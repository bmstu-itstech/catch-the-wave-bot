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
    
    pub fn next_meeting_text(&self, quest: &str) -> String {
        let header = self.current_meeting_header;
        format!("{header}\n\n{quest}")
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
    pub user_not_found: StaticText,
    pub meetings_text: StaticText,
    pub meetings_quests_button: StaticText,
    pub meetings_assign_button: StaticText,
    pub meetings_verify_button: StaticText,
    pub meetings_statistics_button: StaticText,
    pub meetings_promote_button: StaticText,
    pub quests_create_next_button: StaticText,
    pub quests_create_next_text: StaticText,
    pub quests_create_next_success: StaticText,
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
    
    pub fn user_info_text(
        &self, 
        username: &str,
        full_name: &str,
        group_name: &str,
    ) -> String {
        format!(
            "Никнейм в Telegram: @{}\n\
             ФИО: @{}\n\
             Уч. группа: @{}\n",
            username, full_name, group_name
        )
    }
    
    pub fn quests_info(
        &self,
        current_quest_text: Option<String>,
        next_quest_text: Option<String>,
    ) -> String {
        format!(
            "<b>Текущий квест</b>\n\
             {}\n\
             \n\
             <b>Следующий квест</b>\n\
             {}\n",
            current_quest_text.unwrap_or(String::from("-")), 
            next_quest_text.unwrap_or(String::from("-"))
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
        user_not_found: "Пользователь не найден :(",
        meetings_text: "Админ-панель встреч",
        meetings_quests_button: "Задания",
        meetings_assign_button: "Назначить пары",
        meetings_verify_button: "Верифицировать",
        meetings_statistics_button: "Статистика",
        meetings_promote_button: "Обновить задания",
        quests_create_next_button: "Создать следующее",
        quests_create_next_text: "Введите описание задания",
        quests_create_next_success: "Успешно создано!",
    }
};
