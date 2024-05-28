using System.ComponentModel;
using System.Runtime.CompilerServices;
using System.Collections.ObjectModel;

namespace rover_desktop_ui
{
    public partial class HomePage : ContentPage
    {
        private List<string> _consented = new List<string> { "View applications running on your system", "View installed applications", "View list of system users", "Apply policies to applications", "Scan for confidential documents, but not read your documents", "Control where data can be moved from or into this device (e.g, controlling if a file can be placed on a USB, if it can copy/paste)" };
        public List<string> Consented
        {
            get => _consented;
            set
            {
                if (_consented != value)
                {
                    _consented = value;
                    OnPropertyChanged();
                }
            }
        }

        public HomePage()
        {
            InitializeComponent();
            BindingContext = this;

            // Consented.Add("test");
        }

        private void OnCounterClicked(object sender, EventArgs e)
        {
        }
    }

}
