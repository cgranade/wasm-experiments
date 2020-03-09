FROM rust:latest

# Install APT prerequisites.
RUN apt-get update && \
    apt-get -y install \
                       # Dependencies for the .NET Core SDK.
                       wget \
                       pgp \
                       vim \
                       apt-transport-https \
                       # Not strictly needed, but Git is useful for several
                       # interactive scenarios, so we finish by adding it as
                       # well. Thankfully, Git is a small dependency (~3 MiB)
                       # given what we have already installed.
                       git && \
    # We clean the apt cache at the end of each apt command so that the caches
    # don't get stored in each layer.
    apt-get clean && rm -rf /var/lib/apt/lists/

# Trim down the size of the container by disabling the offline package
# cache. See also: https://github.com/dotnet/dotnet-docker/issues/237
ENV NUGET_XMLDOC_MODE=skip \
    DOTNET_SKIP_FIRST_TIME_EXPERIENCE=true

# Now that we have all the dependencies in place, we install the .NET Core SDK itself.
RUN wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > microsoft.asc.gpg && \
    mv microsoft.asc.gpg /etc/apt/trusted.gpg.d/ && \
    wget -q https://packages.microsoft.com/config/debian/9/prod.list && \
    mv prod.list /etc/apt/sources.list.d/microsoft-prod.list && \
    chown root:root /etc/apt/trusted.gpg.d/microsoft.asc.gpg && \
    chown root:root /etc/apt/sources.list.d/microsoft-prod.list && \
    apt-get -y update && \
    apt-get -y install dotnet-sdk-3.1 && \
    apt-get clean && rm -rf /var/lib/apt/lists/

# Install NPM.
RUN curl -sL https://deb.nodesource.com/setup_13.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && rm -rf /var/lib/apt/lists/

# Install rust components used to support the IDE.
RUN rustup component add rls rust-analysis rust-src

# Get required cargo tools.
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
    cargo install cargo-wasi

# Get required NPM packages.
RUN npm install @webassemblyjs/wasm-opt@1.9.0
