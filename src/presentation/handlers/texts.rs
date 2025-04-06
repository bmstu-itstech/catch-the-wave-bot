type StaticText = &'static str;

pub struct RegistrationTexts {
    pub start: StaticText,
    pub enter_full_name: StaticText,
    pub enter_group_name: StaticText,
    pub registration_complete: StaticText,
}

pub struct MeetingTexts {
    pub no_next_meeting: StaticText,
    pub accept_next_meeting: StaticText,
    pub accept_button: StaticText,
    pub after_accept: StaticText,
    pub reject_button: StaticText,
    pub after_reject: StaticText,
    pub current_meeting_header: StaticText,
}

impl MeetingTexts {
    pub fn current_meeting_text(&self, quest: &str, partner: &str) -> String {
        let header = self.current_meeting_header;
        format!("{header}\n\nПартнёр: @{partner}\n\n{quest}")
    }
}

pub struct Menu {
    pub text: StaticText,
    pub next_meeting_button: StaticText,
    pub current_meeting_button: StaticText,
    pub profile_button: StaticText,
    pub rules_button: StaticText,
}

pub struct AdminMenu {
    pub text: StaticText,
    pub users_button: StaticText,
    pub meetings_button: StaticText,
    pub users_query_header: StaticText,
}

impl AdminMenu {
    pub fn users_query_text(&self, usernames: &[&str]) -> String {
        usernames
            .iter()
            .fold(
                String::from(self.users_query_header), 
                |prev, cur| prev + "@" + cur + "\n"
            )
    }
}


pub struct Texts {
    pub registration: RegistrationTexts,
    pub meeting: MeetingTexts,
    pub menu: Menu,
    pub admin_menu: AdminMenu
}

pub const T: Texts = Texts {
    registration: RegistrationTexts {
        start: "Привет-привет, рады видеть тебя с нами на одной волне! 🏄‍♂️",
        enter_full_name: "Для регистрации пожалуйста укажи своё ФИО в формате: СМурфик СМурфович СМурфов",
        enter_group_name: "Твоя учебная группа в формате: СМ11-11Б",
        registration_complete: "Поздравляю, теперь ты точно с нами 🥰",
    },
    meeting: MeetingTexts {
        no_next_meeting: "Упс, кажется, следующей встречи не назначено :(",
        accept_next_meeting: "Подтверди участие в следующей встрече",
        accept_button: "Подтверждаю ✅",
        after_accept: "Вы подтвердили участие в следующей встрече!",
        reject_button: "Не смогу участвовать ❌",
        after_reject: "Вы отказались от участия в следующей встречу(",
        current_meeting_header: "Задание на следующую встречу",
    },
    menu: Menu {
        text: "Меню",
        next_meeting_button: "Следующая встреча ❤️‍🩹",
        current_meeting_button: "Актуальная встреча 💌",
        profile_button: "Профиль 🧐",
        rules_button: "Правила ❓",
    },
    admin_menu: AdminMenu {
        text: "Меню администратора",
        users_button: "Пользователи",
        meetings_button: "Встречи",
        users_query_header: "Все пользователи бота",
    }
};
