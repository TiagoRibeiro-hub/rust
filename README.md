### calculator:
- Calculator using Reverse Polish Notation
#### how to run:
- Allowed characters: + - * / ^ ( )

- --calc "{expression}"
    - (--s) shows rpn 

### process images:
#### how to run:
- --img {file_path}
    - (--cs) color scale 
        - (--gs) gray scale 
        - (--bs) blue scale 
        - (--grs) green scale 
        - (--rs) red scale
            - (--G "1.0") gama, value between 0.0 (darken) and 1.0 (brighten) - default 1.0
    - (--p) pixelated 
        - (--D "5,5") dimensions, (width,height) - default (5,5)
    - (--a) ascii
    - (--o) {file_path_to_save}

- exemple: 
    - pixelate => --img "../imgs/chestnut_tailed_starling.jpg" --p --D  "5,5"" --o "../imgs/"
    - pixelate => --img "../imgs/chestnut_tailed_starling.jpg" --p  --o "../imgs/"

    - colors => --img "../imgs/chestnut_tailed_starling.jpg" --cs --gs --G "0.8" --o "../imgs/"
    - colors => --img "../imgs/chestnut_tailed_starling.jpg" --cs --gs --o "../imgs/"
 