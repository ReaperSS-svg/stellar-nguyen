# stellar-nguyen
<img width="1297" height="703" alt="image" src="https://github.com/user-attachments/assets/57881ab9-656a-4008-8075-28f79fbf707d" />
# 🛒 Grocery Inventory Smart Contract (Stellar/Soroban)

Một Smart Contract tối ưu trên mạng lưới Stellar dành cho các tiệm tạp hóa, giúp giải quyết triệt để bài toán quản lý luồng nhập (Inflow) và xuất (Outflow) hàng hóa.

Thay vì lưu trữ sổ sách thủ công dễ sai sót, contract này biến mọi dữ liệu tồn kho thành nguồn sự thật duy nhất (Single Source of Truth) bất biến trên blockchain, đồng thời tự động tính toán Giá vốn hàng bán (COGS) và Lợi nhuận (Profit) theo phương pháp **Bình quân gia quyền (Weighted Average Cost - WAC)**.

---

## 📍 Thông tin Triển khai (Deployment)

* **Mạng lưới:** Stellar Testnet
* **Ngôn ngữ:** Rust (Soroban SDK)
* **Contract ID:** `CBLW7EPJVESMGU2ZLBR3RZVBLCJS4MFNBOBJBPUFVREMSXLMN7WNMZWY`

---

## ⚙️ Các hàm chức năng cốt lõi (Methods)

Smart Contract cung cấp 3 hàm cơ bản, bám sát nghiệp vụ thực tế của một cửa hàng bán lẻ:

1. **`inflow(item_id, qty, unit_cost)` - Nhập kho**
   * **Mô tả:** Ghi nhận số lượng hàng nhập và giá nhập. Contract sẽ tự động cộng dồn số lượng và tổng giá trị vốn của mặt hàng đó vào kho.
   * **Tham số:** Mã hàng (`Symbol`), Số lượng (`u32`), Giá nhập 1 đơn vị (`u32`).

2. **`outflow(item_id, qty, unit_price)` - Xuất bán & Tính lãi**
   * **Mô tả:** Trừ số lượng hàng bán ra khỏi kho. Dựa vào tổng vốn hiện tại, contract tự động tính toán COGS (Giá vốn hàng bán) và lợi nhuận thu được từ giao dịch này.
   * **Tham số:** Mã hàng (`Symbol`), Số lượng bán (`u32`), Giá bán 1 đơn vị (`u32`).
   * **Kết quả trả về:** `(COGS, Profit)`

3. **`get_stock(item_id)` - Xem tồn kho**
   * **Mô tả:** Truy vấn dữ liệu tồn kho theo thời gian thực.
   * **Kết quả trả về:** `ItemState { quantity, total_cost }`

---

## 🚀 Tương tác nhanh qua Stellar CLI

Ví dụ gọi hàm kiểm tra tồn kho của mặt hàng `HAOHAO`:
```bash
stellar contract invoke \
  --id CBLW7EPJVESMGU2ZLBR3RZVBLCJS4MFNBOBJBPUFVREMSXLMN7WNMZWY \
  --network testnet \
  --source <TÊN_VÍ_CỦA_BẠN> \
  -- \
  get_stock \
  --item_id HAOHAO
