namespace rover_desktop_ui
{
    public partial class MainPage : ContentPage
    {
        public MainPage()
        {
            InitializeComponent();
        }

        private void OnCounterClicked(object sender, EventArgs e)
        {
        }

        private void OnRegisterPageClicked(object sender, EventArgs e)
        {
            Navigation.PushAsync(new Register_your_device());
        }

        private void OnHomePageClicked(object sender, EventArgs e)
        {
            Navigation.PushAsync(new HomePage());
        }
    }
}
