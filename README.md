# RustConsoleStudy


いろいろ気づいたことに関してメモ  

### クラスに関して
 ***  
 ・C++とかにあるような、クラスは存在しない。  
   
   別にこんな感じにトレイト？だったりクレート？だったりで分けなくてもmain.rsに全部書いても問題はない。  
   この書き方はディレクトリまで分けたい場合の書き方になる  
   
   実装例  
     
   ディレクトリ構造(targetフォルダの下など 省略あり  
     
   example_class(root)  
   L　.gitignore  
   L　Cargo.lock  
   L　Cargo.toml  
   L　target  
   L　src  
   　L lib.rs    
   　L main.rs  
   　L hoge  
   　　L mod.rs  
   
mod.rs   
```
//構造体をまずつくる
struct hoge{
  foo: f64,
}
  
//implをつかってトレイトを作る
impl hoge
  //トレイトからのみつかえる。初期化用
  pub fn new(foo: f64) -> hoge{
    hoge{ foo : foo }
  }
      
  //&self,&mut selfを付けると変数から使用できる
  pub fn bar(&mut self){
  }
       
  pub fn fuga(&self){
  }
}
```
  
lib.rs
```
//module。src/hoge.rsかsrc/hoge/mod.rsのどちらかを探す？
pub mod hoge;
```
使用時はこんな感じ  
   
main.rs   
```
extern crate example_class       
use example_class::hoge::hoge;
      
let mut val = hoge::new(0.0);  
      
val.bar();
val.fuga();
```  
