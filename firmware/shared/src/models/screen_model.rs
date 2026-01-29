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
