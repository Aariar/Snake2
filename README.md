# Snake2
[AariaToys](https://github.com/Aariar/bevy_games/tree/main/AariaToys)の第2作、[ヘビゲーム](https://github.com/marcusbuffett/bevy_snake)から派生したBevy製ゲームで[Snake](https://github.com/Aariar/snake)の第2弾です。  
<img src="https://1.bp.blogspot.com/-q2bYPY2eJUM/X60G2pH0rHI/AAAAAAAABD0/-vc3_B2NL28MbK2K8lpoZg7HafmAttZiwCLcBGAsYHQ/s320/snake2.png" width=320>
## Game rules
前作の[Snake](https://github.com/Aariar/snake)と比べ、以下のような変更があります。
- 移動がグリッド単位でなくピクセル単位となったことで、操作感(ゲーム性)が大きく変わりました([Snake](https://github.com/Aariar/snake)と好みは分かれる部分かも知れません)。
- 移動キーは"WASD"に変更しました(コードを直接いじることで、簡単に前作と同じ操作に戻せます)。
- Spaceキーにてリセットできます(前作はEnterキーでした)。
- コンソール画面にScore(餌取得数)とFood(残っている餌の数)が表示されます(餌の出現と同時に表示更新されます)。
- 背景、餌、ヘビの頭と尾の色を変更しました(コードを直接いじることで自由に変更可能です)。
- コード自体に多めにコメントをつけました。Rust自体を知らなくても、見様見真似で簡単な部分は改変を加えることができるかと思います。
- 土台の[Bevy](https://bevyengine.org/)が2.0から3.0にバージョンアップしました(処理性能が上がっているような気がします)。
- 操作変更する場合、コードの「keyboard_input.pressed(KeyCode::xxx)」部分をいじることになりますが、[KeyCode](https://docs.rs/bevy/0.3.0/bevy/prelude/enum.KeyCode.html)を参照してみてください。
- 以下に示すconfig.txtの内容にも変更があります。

## Customize
[config.txt](https://github.com/Aariar/Snake2/blob/main/config.txt)にてゲーム設定を自由に調整することができます(ゲーム再起動で反映)。  
半角:の後の値(数値かbool値)部分のみ書き換えます。簡易な処理をしているため、「:値(空白不可)」以外はあまりいじらないでください。  
具体的な処理は[config_load](https://github.com/Aariar/Snake2/blob/main/src/main.rs)から確認できます。  
  
- snake_width ： ヘビの頭の横サイズ(pixel)を指定します。最低でも3以上必要です。
- snake_height ： ヘビの頭の縦サイズ(pixel)を指定します。最低でも3以上必要です。
- snake_speed ： ヘビの移動速度を指定します。この部分のみ小数点(例:1.5)が使えます。
- win_width ： ゲーム画面の横サイズ(pixel)を指定します。
- win_height ： ゲーム画面の縦サイズ(pixel)を指定します。
- food_width ： 餌の横サイズ(pixel)を指定します。
- food_height ： 餌の縦サイズ(pixel)を指定します。
- food_pop ： 餌の出現頻度を(1/1000秒)単位で指定します。このタイミングでコンソールのスコア表示が更新されます。
- tail_shrink ： ここを「true」にすると、止まっている間、ヘビの尾が収縮していきます。
