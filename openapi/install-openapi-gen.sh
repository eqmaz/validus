#!/bin/bash

# Install Java Runtime Environment (JRE)
echo "Installing Java Runtime Environment..."
sudo apt update
sudo apt install -y default-jre

# Define variables
OPENAPI_VERSION="7.4.0"
JAR_URL="https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/${OPENAPI_VERSION}/openapi-generator-cli-${OPENAPI_VERSION}.jar"
JAR_PATH="/usr/local/bin/openapi-generator-cli.jar"
WRAPPER_PATH="/usr/local/bin/openapi-generator"

# Download OpenAPI Generator CLI JAR
echo "Downloading OpenAPI Generator CLI version ${OPENAPI_VERSION}..."
wget "$JAR_URL" -O openapi-generator-cli.jar

# Move the JAR file to /usr/local/bin
echo "Moving OpenAPI Generator CLI JAR to ${JAR_PATH}..."
sudo mv openapi-generator-cli.jar "$JAR_PATH"

# Make the JAR file executable
echo "Making the JAR file executable..."
sudo chmod +x "$JAR_PATH"

# Create a wrapper script
echo "Creating wrapper script at ${WRAPPER_PATH}..."
sudo bash -c "cat > '$WRAPPER_PATH' <<'EOL'
#!/bin/bash
exec java -jar /usr/local/bin/openapi-generator-cli.jar \"\$@\"
EOL"

# Make the wrapper script executable
echo "Making the wrapper script executable..."
sudo chmod +x "$WRAPPER_PATH"

# Verify installation
echo "Verifying installation..."
"$WRAPPER_PATH" version

echo "Installation and setup complete. You can now use 'openapi-generator' command."
