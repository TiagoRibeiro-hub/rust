### calculator:
- Calculator using Reverse Polish Notation
#### how to run:
- Allowed characters: + - * / ^ ( )

- --calc "{expression}"
    - (--s) shows rpn 

### process images:
#### how to run:
- --img {file_path}
    - (--gs) grey scale (--G "1.0") gama, value between 0.0 and 1.0 - default 1.0
    - (--p) pixelated (--D "5,5") dimensions, (width,height) - default (5,5)
    - (--a) ascii
        - (--o) file path to save

- exemple: --img "./image.png" --gs --G "0.8" --o "./"
 