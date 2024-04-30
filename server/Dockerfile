# Use the official Rust image as the base
FROM rust:latest
WORKDIR /rover-server

ENV DATABASE_URL="mysql://example-user:my_cool_secret@127.0.0.1:3306/prod"
ENV smtp_host="smtp.sendgrid.net"
ENV smtp_port="587"
ENV smtp_username="apikey"
ENV smtp_from_header="noreply@paperplane.motionfans.com"

# Install libcap2-bin for setting capabilities
RUN apt update
RUN apt install -y libcap2-bin default-mysql-client dnsutils msmtp

# Set the capability to bind to port 80 for the cargo binary
RUN setcap 'cap_net_bind_service=+ep' /usr/local/cargo/bin/cargo

# Add a non-root user kube with restricted shell
RUN adduser kube --disabled-login
RUN usermod -s /bin/rbash kube

# RUN echo "defaults\nauth on\ntls on\ntls_starttls on\n\naccount smtp\nhost $smtp_host\nport $smtp_port\nfrom $smtp_from_header\nuser $smtp_username\npassword $smtp_password\n\n# Set default\naccount default : smtp" > /home/kube/.msmtprc
# RUN chown -R kube /home/kube/.msmtprc
# RUN chmod 600 /home/kube/.msmtprc

# RUN echo $smtp_password > /home/kube/.msmtp-password.gpg
# RUN chown -R kube /home/kube/.msmtp-password.gpg

# Remove the dummy code and copy the real code
RUN rm -f target/release/deps/rover-server*
COPY . /rover-server

# Build
RUN cargo build --release

# Expose port 80 for the web server
EXPOSE 8000

# Remove libcap2-bin and clean up apt cache
RUN apt remove -y libcap2-bin
RUN apt autoremove -y
RUN apt clean

# Run the application as kube user
USER kube
CMD echo "defaults\nauth on\ntls on\ntls_starttls on\n\naccount smtp\nhost $smtp_host\nport $smtp_port\nfrom $smtp_from_header\nuser $smtp_username\npassword $smtp_password\n\n# Set default\naccount default : smtp" > /home/kube/.msmtprc && chmod 600 /home/kube/.msmtprc && ./target/release/rover-server