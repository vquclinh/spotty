truoc tien, tai rust thong qua rustup:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo: quan ly code, thu vien
rustc: trinh bien dich cua Rust
rustup: trinh quan ly phien ban

1) cargo init -> Cargo.toml & Cargo.lock == package.json

2) crate: main.rs & lib.rs (1 package -> 1 lib crate)
co the co nhieu binary crate, khi chay chi can chi dinh chay crate nao thong qua flag --bin
1 package chi co 1 lib crate, neu muon co nhieu lib crate thi phai co cargo workspace lon bao boc ben ngoai

3) File Cargo.toml: [package] [dependencies]
   crates.io

4) O day t setup project co cau truc 1 lib + 1 binary crate thoi vi app minh chi co' 1 ung dung, 1 phan mem thoi, cac folder app, event, handlers nhu cac features se support cho library crate duy nhat cua project

5) config.rs: de ap dung config cua nguoi dung (thong qua yaml)

6) redirect_uri.rs: lay token tu Spotify API gui ve khi dang nhap vao Spotify

7) Widget == Components (ratatui == shadcn-ui)
      ti`m widget tren ratatui: https://ratatui.rs/concepts/widgets/ (day la cac widget mac dinh, chi can xem cach su dung roi su dung thoi)

      con neu muon dung cac widget nang cao duoc cong dong phat trien thi truy cap: https://crates.io/search?q=ratatui. Sau do' va`o: https://docs.rs de doc cach su dung. Xong roi chi can cargo add ... roi use...

      ***** 
      repo tong hop tat ca widget: https://github.com/ratatui/awesome-ratatui

8) crossterm giong kieu Event Listener

9) reqwest == axios (get, post, put, delete API)

10) rspotify: wrapper spotify API, giup ghi ngan lenh lai

11) dirs: phat hien he dieu hanh cua nguoi dung la gi de dua file config vao dung cho

12) serde_yaml & serde_json: doc file yaml va json

13) unicode-width: tinh toan chinh xac do rong, de ve TUI khong bi vo

14) arboard: cung cap quyen truy cap vao clipboard

15) sau nay co the tim cac thu vien neu can tai: https://lib.rs/ va doc document tai: https://doc.rs/ 

16) hoac co the tham khao them nhieu thu vien huu ich tai: https://github.com/LargeModGames/spotatui/blob/main/Cargo.toml (repo ca' nhan)

tong hop hau nhu cac thu vien huu ich:
https://github.com/ratatui/awesome-ratatui

xem muc: Command-line, Network programming, Audio, Encoding, Logging,...

17) extension: rust-analyzer

18) 
Bước 1 — App struct         nền tảng, mọi thứ phụ thuộc vào đây
Bước 2 — Event loop         vòng lặp chính, giữ app chạy
Bước 3 — UI cơ bản          render màn hình trống trước
Bước 4 — Spotify Auth       đăng nhập, lấy token
Bước 5 — Network            gọi API lấy data
Bước 6 — Handlers           xử lý phím bấm
Bước 7 — Tính năng thực     playlist, player, search...