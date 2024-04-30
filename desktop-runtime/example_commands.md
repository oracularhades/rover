// Reminder: Require admin to install Rover or change settings.

# On-boarding / off-boarding
rover login --hostname rover.motionfans.com
rover logout [optional: --emergency-discharge=197314]

# Sync / Status
rover sync
rover status

# Safety key
// A safety key is a public-key stored in the device at setup. Admins have private-keys used to sign sensitive requests (such as initiating screen-sharing or installing applications) to heavily mitigate damage in the event of a server breach.
rover safety-key status
rover safety-key set

# Permissions
rover permissions
rover permissions update
rover permissions list-possible
^ All permissions you could possibly give.

# Browser isolation
rover browser-isolation setup
rover browser-isolation uninstall
rover browser-isolation exceptions
rover browser-isolation add-exception example.com
rover browser-isolation remove-exception example.com

# Screen-share
rover screen-share status
^ Shows information such as, if an admin can access the device without a yes / no popup to the user, if screensharing requires a safety key set at Rover onboarding. A "an admin was here" popup that can only be dismissed via a physical USB mouse will always show after an admin has connected to screen-sharing.
rover screen-share activity-log
rover screen-share accept [code]
rover screen-share block [code]

# Uninstall
rover uninstall