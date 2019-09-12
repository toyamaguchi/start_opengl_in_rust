Start OpenGL in Rust (RustではじめるOpenGL)
==================================================

(日本語の解説は英文の後にあります。 Japanese follows English.)

This repository includes sample code for the book "Start OpenGL in Rust".

- [(Paper Edition) Start OpenGL in Rust](https://toyamaguchi.booth.pm/items/1557513)
- [(Download Edition) Start OpenGL in Rust](https://toyamaguchi.booth.pm/items/1557536)
- [(Download Edition with Pasword) Start OpenGL in Rust](https://toyamaguchi.booth.pm/items/1552481)

All directories are associated to each chapters.

- [001_dev_env](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/001_dev_env): Preparation of Development Environment
    - This is a "Hello, World!" program created by "cargo new" command.
- [002_sdl](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/002_sdl): SDL
    - This is a progmram which shows a window by SDL. It can quit by Esc key.
- [003_opengl](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/003_opengl): OpenGL
    - This program renders a triangle in the window. It uses 2 shaders with model matrix, view matrix, and projection matrix.
- [004_imgui](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/004_imgui): Dear ImGui
    - This program renders a window and some widgets in OpenGL rendering area by Dear ImGui.
- [005_3d_object](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/005_3d_object): 3Dオブジェクト
    - This program renders a cube. A debug window has some widgets to control parameters.
- [006_texture](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/006_texture): テクスチャ
    - This program renders a cube with textures. Lights can be controled by widgets.


---

このレポジトリは、書籍『RustではじめるOpenGL』で扱っているサンプルコードを収めています。

- [(紙版) RustではじめるOpenGL](https://toyamaguchi.booth.pm/items/1557513)
- [(DL版) RustではじめるOpenGL](https://toyamaguchi.booth.pm/items/1557536)
- [(パスワード付きDL版) RustではじめるOpenGL](https://toyamaguchi.booth.pm/items/1552481)

それぞれのディレクトリは、各章に紐付いています。

- [001_dev_env](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/001_dev_env): 開発環境の準備
    - 「cargo new」してできあがった「Hello, World!」を表示するプログラムです。
- [002_sdl](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/002_sdl): SDL
    - SDLを使ってウィンドウの表示をします。キーボードのイベントを所得して、Escキーで終了します。
- [003_opengl](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/003_opengl): OpenGL
    - ウィンドウ内に三角形を表示します。2種類のシェーダや、モデル行列、ビュー行列、射影行列を使います。
- [004_imgui](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/004_imgui): Dear ImGui
    - Dear ImGuiを使って、OpenGLの描画領域にウィンドウやウィジェットを描画します。
- [005_3d_object](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/005_3d_object): 3Dオブジェクト
    - 立方体の描画をします。デバッグ用のウィジェットでパラメータの切り替えができるようにします。
- [006_texture](https://github.com/toyamaguchi/start_opengl_in_rust/tree/master/006_texture): テクスチャ
    - 立方体にテクスチャを貼ります。照明の光も導入し、ウィジェットで操作できるようにします。
