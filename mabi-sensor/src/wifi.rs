use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};

const SSID: Option<&str> = option_env!("WIFI_SSID");
const PASSWORD: Option<&str> = option_env!("WIFI_PASSWORD");

pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID
            .expect("Environment variable not set")
            .try_into()
            .unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD
            .expect("Environment variable not set")
            .try_into()
            .unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    log::info!("Wifi started");

    wifi.connect()?;
    log::info!("Wifi connected");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up");

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    log::info!("Wifi DHCP info: {:?}", ip_info);

    Ok(())
}
