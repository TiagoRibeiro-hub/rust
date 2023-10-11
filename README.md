### calculator:
- Calculator using Reverse Polish Notation
#### how to run:
- Allowed characters: + - * / ^ ( )

- (--calc) "{expression}"
    - (--s) shows rpn 

### process images:
#### how to run:
- (--img) {file_path}
    - (--cs) color scale 
        - (--gs) gray scale 
        - (--bs) blue scale 
        - (--grs) green scale 
        - (--rs) red scale
            - (--g "1.0") value between 0.0 (darken) and 1.0 (brighten) - default 1.0
    - (--p) pixelated 
    - (--r) resize
        - (--b) bilinear interpolation
        - (--c) bicubic interpolation
            - (--d) dimension "1000,569"

- (--o) {file_path_to_save}
- (--n) {file_name}, without extension

- exemple: 
    - pixelate => --img "../imgs/chestnut_tailed_starling.jpg" --p  --o "../imgs/"
    - pixelate => --img "../imgs/chestnut_tailed_starling.jpg" --p  --o  "../imgs/" -n pixelate

    - colors => --img "../imgs/chestnut_tailed_starling.jpg" --cs --gs --g "0.8" --o "../imgs/"
    - colors => --img "../imgs/chestnut_tailed_starling.jpg" --cs --gs --o "../imgs/" --n blue_scale
    
    - resize => --img "../imgs/chestnut_tailed_starling.jpg" --r --b --d 1000,796 --o "../imgs/"
    - resize => --img "../imgs/chestnut_tailed_starling.jpg" --r --b --o "../imgs/" --n teste
 