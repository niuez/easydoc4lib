# easydoc4lib

ライブラリの簡易的なドキュメント生成をします.(自分用)

- `easydoc4lib.toml`に設定を書きます. このファイルがあるディレクトリで実行します.
  - `regex`: ドキュメント生成したいファイルの正規表現を書きます. マッチしたファイルだけがドキュメント生成されます.
    - 例) `[.]*.hpp`
  - `out_dir`: ドキュメント生成先を指定します.
    - 例) `./md`
- `/**`から始まる行から, `**/`で終わる行までがそのまま抽出されます.
- `@数字@`とすると, ドキュメントコメント終わりの次の行から, 数字だけ先の行のコードを埋め込みます.
- `@-数字@`とすると, ドキュメントコード始まりの行から, 数字だけ前の行のコードを埋め込みます.

## Example

```cpp
#include <iostream>
using namespace std;
/**
 * ```cpp
 * @-1@
 * ```
 *
 * おまじない
 *
 * ```cpp
 * @0@
 * ```
 * here we go
 **/
int main(){
  std::cout << "hello world" << std::endl;
}
```
↓
これが, こうなる
↓

```cpp
using namespace std;
```

おまじない

```cpp
int main(){
```
here we go
# Code

```cpp
#include <iostream>
using namespace std;

int main(){
  std::cout << "hello world" << std::endl;
}
```
