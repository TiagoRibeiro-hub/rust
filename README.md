### calculator:
- Calculator using Reverse Polish Notation
#### how to run:
- Allowed characters: + - * / ^ ( )

- (--calc) "{expression}"
    - (--s) shows rpn 

### process images:
#### how to run:
- (--img) {file_path}

    - (--cs) color processes 
        - (--gs) gray scale 
        - (--bs) blue scale 
        - (--grs) green scale 
        - (--rs) red scale
        - (--d) darken (--g "20.5") value between 0 and 100
        - (--l) lighten (--g "20.5") value between 0 and 100
        - (--i) invert
        - (--lc) low contrast 
        - (--hc) high contrast

    - (--p) pixelated 

    - (--r) resize
        - (--b) bilinear interpolation
        - (--c) bicubic interpolation
            - (--d) dimension "1000,569"

    - (--f) filter
        - (--s) kernel size just odd numbers, default 21 ( 21*21 kernel )
            (--b) box filter

- (--o) {file_path_to_save}
- (--n) {file_name}, without extension

- exemple: 
    - pixelate => --img "../imgs/chestnut_tailed_starling.jpg" --p  --o "../imgs/"
    - pixelate => --img "../imgs/chestnut_tailed_starling.jpg" --p  --o  "../imgs/" -n teste

    - colors => --img "../imgs/chestnut_tailed_starling.jpg" --cs --gs --o "../imgs/"
    - colors => --img "../imgs/chestnut_tailed_starling.jpg" --cs --i --o "../imgs/" --n teste
    - colors => --img "../imgs/chestnut_tailed_starling.jpg" --cs --d --g 25.5 --o "../imgs/" --n teste

    - resize => --img "../imgs/chestnut_tailed_starling.jpg" --r --b --d 1000,796 --o "../imgs/"
    - resize => --img "../imgs/chestnut_tailed_starling.jpg" --r --b --d 1200,496 --o "../imgs/" --n teste

    - filter_box => --img "../imgs/chestnut_tailed_starling.jpg" --f --b --o "../imgs/" 
    - filter_box => --img "../imgs/chestnut_tailed_starling.jpg" --f --b --s 7 --o "../imgs/" --n teste
 