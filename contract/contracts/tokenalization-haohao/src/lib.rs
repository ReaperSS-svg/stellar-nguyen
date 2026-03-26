#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol};

// Cấu trúc dữ liệu lưu trữ cho từng mặt hàng
#[contracttype]
#[derive(Clone, Default)]
pub struct ItemState {
    pub quantity: u32,   // Số lượng tồn kho
    pub total_cost: u32, // Tổng giá trị vốn của toàn bộ hàng tồn
}

#[contract]
pub struct GroceryInventory;

#[contractimpl]
impl GroceryInventory {
    
    /// 1. INFLOW: Nhập hàng vào kho
    /// Truyền vào: mã hàng (item_id), số lượng nhập (qty), giá nhập 1 sản phẩm (unit_cost)
    pub fn inflow(env: Env, item_id: Symbol, qty: u32, unit_cost: u32) {
        // Lấy dữ liệu cũ trong kho, nếu chưa có thì tạo mới (default là 0)
        let mut state: ItemState = env.storage().persistent().get(&item_id).unwrap_or_default();
        
        // Cộng dồn số lượng và tổng vốn
        state.quantity += qty;
        state.total_cost += qty * unit_cost;
        
        // Lưu lại vào blockchain
        env.storage().persistent().set(&item_id, &state);
    }

    /// 2. OUTFLOW: Xuất bán hàng
    /// Truyền vào: mã hàng, số lượng bán, giá bán 1 sản phẩm
    /// Trả về: (Giá vốn COGS, Lợi nhuận Profit)
    pub fn outflow(env: Env, item_id: Symbol, qty: u32, unit_price: u32) -> (u32, i32) {
        let mut state: ItemState = env.storage().persistent().get(&item_id).expect("Loi: Mat hang nay chua tung duoc nhap kho!");
        
        if state.quantity < qty {
            panic!("Loi: Khong du hang ton kho de ban!");
        }

        // Tính Giá vốn hàng bán (COGS) theo trung bình gia quyền
        let avg_cost_per_item = state.total_cost / state.quantity;
        let cogs = avg_cost_per_item * qty;
        
        // Tính doanh thu và lợi nhuận
        let revenue = qty * unit_price;
        let profit = (revenue as i32) - (cogs as i32);

        // Cập nhật lại kho (trừ đi số lượng và giá trị vốn tương ứng)
        state.quantity -= qty;
        state.total_cost -= cogs;
        
        env.storage().persistent().set(&item_id, &state);

        // Trả kết quả về cho ứng dụng Web ghi nhận
        (cogs, profit)
    }

    /// 3. VIEW: Xem tồn kho hiện tại
    pub fn get_stock(env: Env, item_id: Symbol) -> ItemState {
        env.storage().persistent().get(&item_id).unwrap_or_default()
    }
}
stellar contract invoke \
  --id CBLW7EPJVESMGU2ZLBR3RZVBLCJS4MFNBOBJBPUFVREMSXLMN7WNMZWY \
  --source student \
  --network testnet \
  -- \
  outflow \
  --item_id HAOHAO \
  --qty 10 \
  --unit_price 10000