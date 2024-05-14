using System.ComponentModel;
using System.Runtime.CompilerServices;
using System.Collections.ObjectModel;

namespace rover_desktop_ui
{
    public partial class HomePage : ContentView, INotifyPropertyChanged
    {
        public HomePage()
        {
            InitializeComponent();
        }

        private void OnCounterClicked(object sender, EventArgs e)
        {
        }
    }

}
