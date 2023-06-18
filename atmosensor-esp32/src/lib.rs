use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::peripheral;
use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi::BlockingWifi, wifi::EspWifi};

pub mod scd30;

pub fn init_wifi(
    ssid: &str,
    password: &str,
    modem: impl peripheral::Peripheral<P = esp_idf_hal::modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> anyhow::Result<Box<EspWifi<'static>>> {
    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;
    log::info!("Starting wifi...");
    wifi.start()?;
    log::info!("Scanning for {}", ssid);

    let ap_infos = wifi.scan()?;
    let channel = if let Some(ssid_ap_info) = ap_infos.into_iter().find(|elem| elem.ssid == ssid) {
        log::info!(
            "Found configured access point {} on channel {}",
            ssid,
            ssid_ap_info.channel
        );
        ssid_ap_info.channel
    } else {
        anyhow::bail!("Configured access point {} not found during scanning", ssid);
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: password.into(),
        channel: Some(channel),
        auth_method: embedded_svc::wifi::AuthMethod::WPA2Personal,
        ..Default::default()
    }))?;

    log::info!("Connecting to access point...");
    wifi.connect()?;
    log::info!("Waiting for DHCP lease...");
    wifi.wait_netif_up()?;
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    log::info!("Wifi DHCP info: {:?}", ip_info);

    Ok(Box::new(esp_wifi))
}
