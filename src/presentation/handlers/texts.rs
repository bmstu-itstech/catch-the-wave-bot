type StaticText = &'static str;

pub struct RegistrationTexts {
    pub start: StaticText,
    pub enter_full_name: StaticText,
    pub enter_group_name: StaticText,
    pub registration_complete: StaticText,
}

pub struct Texts {
    pub registration: RegistrationTexts,
}

pub const T: Texts = Texts {
    registration: RegistrationTexts {
        start: "Привет-привет, рады видеть тебя с нами на одной волне! 🏄‍♂️",
        enter_full_name: "Для регистрации пожалуйста укажи своё ФИО в формате: СМурфик СМурфович СМурфов",
        enter_group_name: "Твоя учебная группа в формате: СМ11-11Б",
        registration_complete: "Поздравляю, теперь ты точно с нами 🥰",
    },
};
