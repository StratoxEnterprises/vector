# Vector - CN 

jde o fork forknuteho projektu https://github.com/syedriko/vector/tree/syedriko-syslog-codec  

potrebujeteme to kvuli tranformaci zprav do syslogu.  
Tohle repo nebude potreba az budou zmeny ohledne syslog sink soucasti standartni verze Vectoru. Tzn az bude vyresene toto issue:  

https://github.com/vectordotdev/vector/issues/6863

respektive zamerogvany tento pull request:  

https://github.com/vectordotdev/vector/pull/17668




## nativni build

instalace rust a cargo  

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

rust build  

`cargo build` pripadne `cargo build --release`

pusteni:  

`./target/debug/vector --config /cesta/folder/config.yaml` pripadne `./target/release/vector --config /cesta/folder/config.yaml`


## Docker build

jejich skripty a makefile je uplne na hovno. Je to komplikovany jako prace a ani se mi nepodarilo uspesne udelat build.  
Pripravil jsem vlastni multistage Dockerfile build ktery nejdriv udela linux build a nasledne preklopi binarku do runtime docker image.  

build (muze trvat fakt dlouho) :  
`docker build -t vector-cn -f Dockerfile .`

pusteni:    
`docker run -d   -v /Users/sracka/Documents/vector.yaml:/etc/vector/vector.yaml:ro  -p 8686:8686 vector-cn:latest  '--config' '/etc/vector/vector.yaml'`