use viewy::prelude::*;

#[get("/picker-select")]
pub fn picker_select_demo() -> Page<'static> {
    Page::with_title("Viewy showcase – Picker & Select").with_content({
        let mut page = VStack::new(Alignment::Stretch);
        page.gap(vec![scale(5)]).padding(vec![scale(5)]);

        page.append_child(Text::new("Picker & Select", TextStyle::H1));

        let mut section_select = VStack::new(Alignment::Stretch);
        section_select
            .as_card(CardStyle::OutlinedRaised)
            .padding(vec![scale(4)])
            .gap(vec![scale(4)])
            .append_child(Text::new("Select", TextStyle::H2));

        let mut select_country = Select::new("country", "fr");
        select_country
            .label("Country")
            .placeholder("Select a country");

        let mut option_fr = SelectOption::new("France", "fr");
        option_fr.icon(Lucide::MapPin);

        let mut option_us = SelectOption::new("United States", "us");
        option_us.icon(Lucide::Flag);

        let mut option_jp = SelectOption::new("Japan", "jp");
        option_jp.icon(Lucide::Mountain);

        select_country
            .append_option(option_fr)
            .append_option(option_us)
            .append_option(option_jp);

        section_select.append_child(select_country);

        let mut select_team = Select::new("team_member", "");
        select_team
            .label("Assignee")
            .placeholder("Search an assignee")
            .required();

        let mut backend = SelectGroup::new("Backend");
        backend
            .append_option(SelectOption::new("Ana", "ana"))
            .append_option(SelectOption::new("Marc", "marc"));

        let mut frontend = SelectGroup::new("Frontend");
        frontend
            .append_option(SelectOption::new("Sofia", "sofia"))
            .append_option(SelectOption::new("Noah", "noah"));

        select_team.append_group(backend).append_group(frontend);
        section_select.append_child(select_team);

        let mut select_nosearch = Select::new("timezone", "utc");
        select_nosearch
            .label("Timezone")
            .placeholder("Choose timezone")
            .disable_search_bar();
        select_nosearch
            .append_option(SelectOption::new("UTC", "utc"))
            .append_option(SelectOption::new("Europe/Paris", "paris"))
            .append_option(SelectOption::new("America/New_York", "new_york"));
        section_select.append_child(select_nosearch);

        page.append_child(section_select);

        let mut section_picker = VStack::new(Alignment::Stretch);
        section_picker
            .as_card(CardStyle::OutlinedRaised)
            .padding(vec![scale(4)])
            .gap(vec![scale(4)])
            .append_child(Text::new("Picker", TextStyle::H2));

        let mut priority = Picker::new("priority", "normal", PickerStyle::Segmented);
        priority.label("Priority");

        let mut pr_low = PickerOption::new("Low", "low");
        pr_low.icon(Lucide::ArrowDown);

        let mut pr_normal = PickerOption::new("Normal", "normal");
        pr_normal.icon(Lucide::Minus);

        let mut pr_high = PickerOption::new("High", "high");
        pr_high.icon(Lucide::ArrowUp);

        priority
            .append_option(pr_low)
            .append_option(pr_normal)
            .append_option(pr_high);

        section_picker.append_child(priority);

        let mut channels = Picker::new("channels", "", PickerStyle::Segmented);
        channels.label("Notifications").multiple();

        let mut ch_email = PickerOption::new("Email", "email");
        ch_email.selected();

        let mut ch_push = PickerOption::new("Push", "push");
        ch_push.selected();

        channels
            .append_option(ch_email)
            .append_option(ch_push)
            .append_option(PickerOption::new("SMS", "sms"));

        section_picker.append_child(channels);

        let mut layout_picker = Picker::new("layout_mode", "comfortable", PickerStyle::Segmented);
        layout_picker.label("Layout mode").vertical();
        layout_picker
            .append_option(PickerOption::new("Compact", "compact"))
            .append_option(PickerOption::new("Comfortable", "comfortable"))
            .append_option(PickerOption::new("Spacious", "spacious"));
        section_picker.append_child(layout_picker);

        let mut payment = Picker::new("payment_method", "card", PickerStyle::RadioGroup);
        payment.label("Payment method").required();

        let mut online = PickerGroup::new("Online");
        online
            .append_option(PickerOption::new("Card", "card"))
            .append_option(PickerOption::new("PayPal", "paypal"));

        let mut offline = PickerGroup::new("Offline");
        offline
            .append_option(PickerOption::new("Bank transfer", "transfer"))
            .append_option(PickerOption::new("Cash", "cash"));

        payment.append_group(online).append_group(offline);
        section_picker.append_child(payment);

        page.append_child(section_picker);

        page
    })
}
