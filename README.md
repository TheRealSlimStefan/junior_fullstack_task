# junior_fullstack_task

## Instalation

If you want to start this project firstly you need to make sure you have rust installed on your computer with nessesary c++ build tools.

Install rust: https://www.rust-lang.org/tools/install \
Install c++ build tools: https://visualstudio.microsoft.com/visual-cpp-build-tools/

Then you need to clone this project repository and open its' folder in terminal. 
After doing this use this commands:

1) rustup default nightly
2) cargo build
3) cargo run

## Usage

After running cargo run command our API will start and it will be accessible under http://localhost:8000/ address.

In this particular API we have two endpoints, to pass needed parameters we can do this like in examples under:

http://localhost:8000/calculateDisselUsageForDistance?distance=140&yearOfProduction=2010&fuelUsagePer100KM=6

http://localhost:8000/probabilityOfUnitInjectorFail/vin=1d111v13311a11111
