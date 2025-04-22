type StaticText = &'static str;

pub struct RegistrationTexts {
    pub start: StaticText,
    pub enter_full_name: StaticText,
    pub enter_group_name: StaticText,
    pub registration_complete: StaticText,
}

pub struct Menu {
    pub text: StaticText,
    pub next_task_button: StaticText,
    pub user_task_button: StaticText,
    pub profile_button: StaticText,
    pub rules_button: StaticText,
}

pub struct Rules {
    pub text: StaticText,
}

pub struct NextTask {
    pub text: StaticText,
    pub accept_button: StaticText,
    pub reject_button: StaticText,
    pub accept_success: StaticText,
    pub reject_success: StaticText,
}

pub struct UserTask;

impl UserTask {
    pub fn user_task(
        &self,
        partner_username: &str,
        title: &str,
        description: &str,
    ) -> String {
        format!(
            "⭐️ Вот твоё задание и партнёр на неделю!\n\
             \n\
             <b>Партнёр</b>: @{partner_username}\n\
             <b>Задание</b>: {title}\n\
             <i>{description}</i>",
        )
    }
}

pub struct Profile {
    pub re_register_button: StaticText,
}

impl Profile {
    pub fn profile(
        &self,
        full_name: &str,
        group_name: &str,
        next_task: &str,
        completed_tasks: &i32,
    ) -> String {
        format!(
            "<b>Вот информация о тебе</b>:\n\
             \n\
             <b>Полное имя</b>: {full_name}\n\
             <b>Учебная группа</b>: {group_name}\n\
             <b>Следующая встреча</b>: {next_task}\n\
             <b>Завершено</b>: {completed_tasks}",
        )
    }
}

pub struct AdminMenu {
    pub text: StaticText,
    pub users_button: StaticText,
    pub create_next_task_button: StaticText,
    pub assign_partner_button: StaticText,
    pub verification_button: StaticText,
}

pub struct AdminUsers {
    pub text: StaticText,
    pub no_users: StaticText,
}

impl AdminUsers {
    pub fn user_info(
        &self,
        username: &str,
        full_name: &str,
        group_name: &str,
        next_meeting_state: &str,
        completed_quests: &i32,
    ) -> String {
        format!(
            "<b>Никнейм</b>: @{username}\n\
             <b>ФИО</b>: {full_name}\n\
             <b>Учебная группа</b>: {group_name}\n\
             <b>Следующая встреча</b>: {next_meeting_state}\n\
             <b>Завершено</b>: {completed_quests}",
        )
    }

    pub fn user_info_with_current_meeting(
        &self,
        username: &str,
        full_name: &str,
        group_name: &str,
        next_meeting_state: &str,
        current_meeting_state: &str,
        partner_username: &str,
        completed_quests: &i32,
    ) -> String {
        format!(
            "<b>Никнейм</b>: @{username}\n\
             <b>ФИО</b>: {full_name}\n\
             <b>Учебная группа</b>: {group_name}\n\
             <b>Следующая встреча</b>: {next_meeting_state}\n\
             <b>Текущая встреча</b>: {current_meeting_state}\n\
             <b>Партнёр</b>: @{partner_username}\n\
             <b>Завершено</b>: {completed_quests}",
        )
    }
}

pub struct AdminAssign {
    pub insufficient_users: StaticText,
    pub assign_first: StaticText,
    pub assign_second: StaticText,
    pub assign_success: StaticText,
    pub no_next_task: StaticText,
}

pub struct AdminCreateTask {
    pub enter_title: StaticText,
    pub enter_description: StaticText,
    pub success: StaticText,
    pub already_exists: StaticText,
}

pub struct AdminCompleteTask {
    pub text: StaticText,
    pub no_users: StaticText,
    pub success: StaticText,
}

pub struct Texts {
    pub registration: RegistrationTexts,
    pub menu: Menu,
    pub rules: Rules,
    pub next_task: NextTask,
    pub user_task: UserTask,
    pub profile: Profile,
    pub admin_menu: AdminMenu,
    pub admin_users: AdminUsers,
    pub admin_assign: AdminAssign,
    pub admin_create_task: AdminCreateTask,
    pub admin_complete_task: AdminCompleteTask,
}

pub const T: Texts = Texts {
    registration: RegistrationTexts {
        start: "Привет-привет, рады видеть тебя с нами на одной волне! 🏄‍♂️",
        enter_full_name: "Для регистрации пожалуйста укажи своё ФИО в формате: СМурфик СМурфович СМурфов",
        enter_group_name: "Твоя учебная группа в формате: СМ11-11Б",
        registration_complete: "Поздравляю, теперь ты точно с нами 🥰",
    },
    next_task: NextTask {
        text: "Подтверди участие в следующей встрече",
        accept_button: "Подтверждаю ✅",
        reject_button: "Не смогу участвовать ❌",
        accept_success: "🌊 Вы подтвердили участие в следующей встрече!\n\
                         Скоро здесь появится ваш партнер и задание на неделю",
        reject_success: "Вы отказались от участия в следующей встрече(",
    },
    user_task: UserTask{},
    menu: Menu {
        text: "Меню",
        next_task_button: "Следующая встреча ❤️‍🩹",
        user_task_button: "Актуальная встреча 💌",
        profile_button: "Профиль 🧐",
        rules_button: "Правила ❓",
    },
    rules: Rules {
        text: "Мы рады, что ты с нами на одной волне!🌊\n\
               Чтобы начать принимать участие в проекте, нужно подтвердить участие в разделе: следующая встреча✅\n\
               В разделе актуальная встреча вы можете узнать задание на текущую неделю и напарника, с которым вы должны его выполнить🎉\n\
               После того как вы получили задание, вы должны договориться с напарником о встречи и выполнить его в течение недели. Чтобы подтвердить участие нужно прислать фотоотчет в чаты львят или Live с хэштегом #на_одной_волне☀️\n\
               В разделе статистика вы можете просмотреть информацию о встречах и об участниках с кем вы встретились🥰"
    },
    profile: Profile {
        re_register_button: "Изменить данные",
    },
    admin_menu: AdminMenu {
        text: "Меню администратора",
        users_button: "Пользователи",
        create_next_task_button: "Создать задание",
        assign_partner_button: "Пары",
        verification_button: "Подтверждения",
    },
    admin_users: AdminUsers {
        text: "Вот список всех пользователей бота",
        no_users: "Список пользователей пуст... странно, но надеюсь у программистов был бэкап)",
    },
    admin_assign: AdminAssign {
        insufficient_users: "Недостаточно пользователей, подтвердивших встречу, для назначения пар - должно быть минимум двое",
        assign_first: "Выберите первого пользователя пары",
        assign_second: "Выберите второго пользователя пары",
        assign_success: "Пара успешно назначена",
        no_next_task: "В базе данных нет информации о следующем задании - без этого невозможно назначить партнёров. Добавьте задание в бота и попробуйте снова",
    },
    admin_create_task: AdminCreateTask {
        enter_title: "Введите название задания",
        enter_description: "Введите текст задания",
        success: "Задание на следующую неделю успешно создано",
        already_exists: "Задание на следующую неделю уже существует. Дождитесь начала новой недели для создания следующего",
    },
    admin_complete_task: AdminCompleteTask {
        text: "Выберите пользователя из списка для подтверждения выполнения задания",   
        no_users: "Нет ни одного пользователя с активным текущим заданием. Необходимо сначала создать пару для выполнения задания",
        success: "Задание успешно засчитано",
    }
};
