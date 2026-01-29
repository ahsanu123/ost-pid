use defmt::Format;

#[derive(Clone, Copy, PartialEq, PartialOrd, Format)]
pub enum Screen {
    ShowTemperature,
    ShowSetpoint,
    ShowError,
    ShowProportionalVal,
    ShowDerivativeVal,
    ShowIntegralVal,

    SetSetpoint,
    SetProportionalVal,
    SetDerivativeVal,
    SetIntegralVal,
}

impl Screen {
    pub fn to_string(&self) -> &'static str {
        match self {
            Screen::ShowTemperature => "Temperature",
            Screen::ShowSetpoint => "SetPoint",
            Screen::ShowError => "Error",
            Screen::ShowProportionalVal => "ProportionalVal",
            Screen::ShowDerivativeVal => "DerivativeVal",
            Screen::ShowIntegralVal => "IntegralVal",

            Screen::SetSetpoint => "Set Setpoint",
            Screen::SetProportionalVal => "Set ProportionalVal",
            Screen::SetDerivativeVal => "Set DerivativeVal",
            Screen::SetIntegralVal => "SetIntegralVal",
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Format)]
pub struct UiState {
    pub setpoint: f32,
    pub temperature: f32,
    pub screen: Screen,
    pub output_val: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            setpoint: 30.0,
            temperature: 25.0,
            screen: Screen::ShowTemperature,
            output_val: 450.0,
        }
    }
}
