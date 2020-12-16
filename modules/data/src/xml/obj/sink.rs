pub trait ObjectDataSink {
    type V: ObjectDataSink;
    type B: BuffDataSink;
    type D: DestructibleDataSink;
    type I: InventoryDataSink;
    type M: MinifigDataSink;

    fn push_attr_v(self, v: u32) -> Self::V;
    fn start_buff(self) -> Self::B;
    fn start_dest(self) -> Self::D;
    fn start_inv(self) -> Self::I;
    fn start_mf(self) -> Self::M;
}

pub trait BuffDataSink {
    type E: ObjectDataSink;
    fn end_buff(self) -> Self::E;
}

pub trait DestructibleDataSink {
    type E: ObjectDataSink;
    fn end_dest(self) -> Self::E;

    fn push_attr_ac(self, v: u32) -> Self;
    fn push_attr_am(self, v: u32) -> Self;
    fn push_attr_d(self, v: bool) -> Self;
    fn push_attr_hc(self, v: u32) -> Self;
    fn push_attr_hm(self, v: u32) -> Self;
    fn push_attr_ic(self, v: u32) -> Self;
    fn push_attr_im(self, v: u32) -> Self;
    fn push_attr_imm(self, v: u32) -> Self;
    fn push_attr_rsh(self, v: u32) -> Self;
    fn push_attr_rsi(self, v: u32) -> Self;
}

pub trait InventoryDataSink {
    type E: ObjectDataSink;
    type IB: InventoryBagsDataSink;
    type IG: InventoryGroupsDataSink;
    type II: InventoryItemsDataSink;

    fn end_inv(self) -> Self::E;
    fn push_attr_csl(self, csl: u32) -> Self;
    fn start_bag(self) -> Self::IB;
    fn start_grps(self) -> Self::IG;
    fn start_items(self) -> Self::II;
}

pub trait InventoryBagsDataSink {
    type E: InventoryDataSink;
    type B: InventoryBagDataSink;
    fn end_bag(self) -> Self::E;
    fn start_b(self) -> Self::B;
}

pub trait InventoryBagDataSink {
    type E: InventoryBagsDataSink;
    fn push_attr_t(self, v: u32) -> Self;
    fn push_attr_m(self, v: u32) -> Self;
    fn end_b(self) -> Self::E;
}

pub trait InventoryGroupsDataSink {
    type E: InventoryDataSink;
    type G: InventoryGroupDataSink;

    fn end_grps(self) -> Self::E;
    fn start_grp(self) -> Self::G;
}

pub trait InventoryGroupDataSink {
    type E: InventoryGroupsDataSink;

    fn end_grp(self) -> Self::E;
    fn push_attr_id(self, v: String) -> Self;
    fn push_attr_l(self, v: String) -> Self;
    fn push_attr_n(self, v: String) -> Self;
    fn push_attr_t(self, v: u32) -> Self;
    fn push_attr_u(self, v: String) -> Self;
}

pub trait InventoryItemsDataSink {
    type E: InventoryDataSink;
    type I: InventoryItemBagDataSink;

    fn push_attr_nn(self, v: String) -> Self;
    fn start_in(self) -> Self::I;
    fn end_items(self) -> Self::E;
}

pub trait InventoryItemBagDataSink {
    type E: InventoryItemsDataSink;
    type I: InventoryItemDataSink;

    fn push_attr_t(self, v: u32) -> Self;
    fn end_in(self) -> Self::E;
    fn start_i(self) -> Self::I;
}

pub trait InventoryItemDataSink {
    type E: InventoryItemBagDataSink;
    type X: InventoryItemExtraDataSink;

    fn end_i(self) -> Self::E;

    fn push_attr_b(self, v: bool) -> Self;
    fn push_attr_c(self, v: u32) -> Self;
    fn push_attr_eq(self, v: bool) -> Self;
    fn push_attr_id(self, v: u64) -> Self;
    fn push_attr_l(self, v: u32) -> Self;
    fn push_attr_s(self, v: u32) -> Self;
    fn push_attr_sk(self, v: u32) -> Self;

    fn start_x(self) -> Self::X;
}

pub trait InventoryItemExtraDataSink {
    type E: InventoryItemDataSink;

    fn end_x(self) -> Self::E;

    fn push_attr_b(self, v: String) -> Self;
    fn push_attr_ma(self, v: String) -> Self;
    fn push_attr_ub(self, v: String) -> Self;
    fn push_attr_ud(self, v: String) -> Self;
    fn push_attr_ui(self, v: String) -> Self;
    fn push_attr_um(self, v: String) -> Self;
    fn push_attr_un(self, v: String) -> Self;
    fn push_attr_uo(self, v: String) -> Self;
    fn push_attr_up(self, v: String) -> Self;
}

pub trait MinifigDataSink {
    type E: ObjectDataSink;

    fn end_mf(self) -> Self::E;

    fn push_attr_cd(self, v: u32) -> Self;
    fn push_attr_es(self, v: u32) -> Self;
    fn push_attr_ess(self, v: u32) -> Self;
    fn push_attr_hc(self, v: u32) -> Self;
    fn push_attr_hd(self, v: u32) -> Self;
    fn push_attr_hdc(self, v: u32) -> Self;
    fn push_attr_hs(self, v: u32) -> Self;
    fn push_attr_l(self, v: u32) -> Self;
    fn push_attr_lh(self, v: u32) -> Self;
    fn push_attr_ms(self, v: u32) -> Self;
    fn push_attr_rh(self, v: u32) -> Self;
    fn push_attr_t(self, v: u32) -> Self;
}
