use super::super::sink::*;
use super::*;

pub type ObjectStoreDataSink<'a> = &'a mut Object;
pub type DestructibleStoreDataSink<'a> = (&'a mut Object, Destructible);
pub type InventoryStoreDataSink<'a> = (&'a mut Object, Inventory);
pub type MinifigStoreDataSink<'a> = (&'a mut Object, Minifig);

impl<'a> ObjectDataSink for ObjectStoreDataSink<'a> {
    type V = Self;
    type B = ObjectStoreDataSink<'a>;
    type D = DestructibleStoreDataSink<'a>;
    type I = InventoryStoreDataSink<'a>;
    type M = MinifigStoreDataSink<'a>;

    fn push_attr_v(self, v: u32) -> Self::V {
        self.attr_v = v;
        return self;
    }

    fn start_buff(self) -> Self::B {
        return self;
    }

    fn start_dest(self) -> Self::D {
        return (self, Destructible::default());
    }

    fn start_inv(self) -> Self::I {
        return (self, Inventory::default());
    }

    fn start_mf(self) -> Self::M {
        return (self, Minifig::default());
    }
}

impl<'a> BuffDataSink for ObjectStoreDataSink<'a> {
    type E = ObjectStoreDataSink<'a>;

    fn end_buff(self) -> Self::E {
        return self;
    }
}

impl<'a> DestructibleDataSink for DestructibleStoreDataSink<'a> {
    type E = ObjectStoreDataSink<'a>;

    fn end_dest(self) -> Self::E {
        let obj = self.0;
        obj.dest = Some(self.1);
        return obj;
    }

    fn push_attr_ac(mut self, ac: u32) -> Self {
        self.1.attr_ac = Some(ac);
        return self;
    }

    fn push_attr_am(mut self, am: u32) -> Self {
        self.1.attr_am = Some(am);
        return self;
    }

    fn push_attr_d(mut self, d: bool) -> Self {
        self.1.attr_d = Some(d);
        return self;
    }

    fn push_attr_hc(mut self, hc: u32) -> Self {
        self.1.attr_hc = Some(hc);
        return self;
    }

    fn push_attr_hm(mut self, hm: u32) -> Self {
        self.1.attr_hm = Some(hm);
        return self;
    }

    fn push_attr_ic(mut self, ic: u32) -> Self {
        self.1.attr_ic = Some(ic);
        return self;
    }

    fn push_attr_im(mut self, im: u32) -> Self {
        self.1.attr_im = Some(im);
        return self;
    }

    fn push_attr_imm(mut self, imm: u32) -> Self {
        self.1.attr_imm = Some(imm);
        return self;
    }

    fn push_attr_rsh(mut self, rsh: u32) -> Self {
        self.1.attr_rsh = Some(rsh);
        return self;
    }

    fn push_attr_rsi(mut self, rsi: u32) -> Self {
        self.1.attr_rsi = Some(rsi);
        return self;
    }
}

impl<'a> InventoryDataSink for InventoryStoreDataSink<'a> {
    type E = ObjectStoreDataSink<'a>;
    type IB = InventoryStoreDataSink<'a>;
    type IG = InventoryStoreDataSink<'a>;
    type II = InventoryStoreDataSink<'a>;

    fn end_inv(self) -> Self::E {
        let obj = self.0;
        obj.inv = Some(self.1);
        return obj;
    }

    fn push_attr_csl(mut self, csl: u32) -> Self {
        self.1.attr_csl = Some(csl);
        return self;
    }

    fn start_bag(self) -> Self::IB {
        return self;
    }

    fn start_grps(self) -> Self::IG {
        return self;
    }

    fn start_items(self) -> Self::II {
        return self;
    }
}

pub type InventoryBagStoreDataSink<'a> = (&'a mut Object, Inventory, Bag);

impl <'a> InventoryBagsDataSink for InventoryStoreDataSink<'a> {
    type E = InventoryStoreDataSink<'a>;
    type B = InventoryBagStoreDataSink<'a>;

    fn end_bag(self) -> Self::E {
        return self;
    }

    fn start_b(self) -> Self::B {
        return (self.0, self.1, Bag::default());
    }
}

impl <'a> InventoryBagDataSink for InventoryBagStoreDataSink<'a> {
    type E = InventoryStoreDataSink<'a>;

    fn end_b(self) -> Self::E {
        let mut inv = self.1;
        inv.bag.push(self.2);
        return (self.0, inv);
    }

    fn push_attr_t(mut self, v: u32) -> Self {
        self.2.attr_t = v;
        return self;
    }

    fn push_attr_m(mut self, v: u32) -> Self {
        self.2.attr_m = v;
        return self;
    }
}

pub type InventoryGroupStoreDataSink<'a> = (&'a mut Object, Inventory, Group);

impl <'a> InventoryGroupsDataSink for InventoryStoreDataSink<'a> {
    type E = InventoryStoreDataSink<'a>;
    type G = InventoryGroupStoreDataSink<'a>;

    fn end_grps(self) -> Self::E {
        return self;
    }

    fn start_grp(self) -> Self::G {
        return (self.0, self.1, Group::default());
    }
}

impl <'a> InventoryGroupDataSink for InventoryGroupStoreDataSink<'a> {
    type E = InventoryStoreDataSink<'a>;

    fn end_grp(self) -> Self::E {
        let mut inv = self.1;
        inv.grps.push(self.2);
        return (self.0, inv);
    }

    fn push_attr_id(mut self, v: String) -> Self {
        self.2.attr_id = v;
        return self;
    }

    fn push_attr_l(mut self, v: String) -> Self {
        self.2.attr_l = v;
        return self;
    }

    fn push_attr_n(mut self, v: String) -> Self {
        self.2.attr_n = v;
        return self;
    }

    fn push_attr_t(mut self, v: u32) -> Self {
        self.2.attr_t = v;
        return self;
    }

    fn push_attr_u(mut self, v: String) -> Self {
        self.2.attr_u = v;
        return self;
    }
}

pub type InventoryItemBagStoreDataSink<'a> = (&'a mut Object, Inventory, ItemBag);

impl<'a> InventoryItemsDataSink for InventoryStoreDataSink<'a> {
    type E = InventoryStoreDataSink<'a>;
    type I = InventoryItemBagStoreDataSink<'a>;

    fn push_attr_nn(mut self, v: String) -> Self {
        self.1.items.attr_nn = v;
        return self;
    }

    fn start_in(self) -> Self::I {
        return (self.0, self.1, ItemBag::default());
    }

    fn end_items(self) -> Self::E {
        return self;
    }
}

pub type InventoryItemStoreDataSink<'a> = (&'a mut Object, Inventory, ItemBag, Item);

impl<'a> InventoryItemBagDataSink for InventoryItemBagStoreDataSink<'a> {
    type E = InventoryStoreDataSink<'a>;
    type I = InventoryItemStoreDataSink<'a>;

    fn push_attr_t(mut self, v: u32) -> Self {
        self.2.attr_t = v;
        return self;
    }

    fn end_in(self) -> Self::E {
        let mut inv = self.1;
        inv.items.children.push(self.2);
        return (self.0, inv);
    }

    fn start_i(self) -> Self::I {
        return (self.0, self.1, self.2, Item::default());
    }
}

pub type InventoryItemExtraStoreDataSink<'a> = (&'a mut Object, Inventory, ItemBag, Item, ItemExtra);

impl<'a> InventoryItemDataSink for InventoryItemStoreDataSink<'a> {
    type E = InventoryItemBagStoreDataSink<'a>;
    type X = InventoryItemExtraStoreDataSink<'a>;

    fn end_i(self) -> Self::E {
        let mut bag = self.2;
        bag.children.push(self.3);
        return (self.0, self.1, bag);
    }

    fn push_attr_b(mut self, v: bool) -> Self {
        self.3.attr_b = v;
        return self;
    }

    fn push_attr_c(mut self, v: u32) -> Self {
        self.3.attr_c = v;
        return self;
    }

    fn push_attr_eq(mut self, v: bool) -> Self {
        self.3.attr_eq = v;
        return self;
    }

    fn push_attr_id(mut self, v: u64) -> Self {
        self.3.attr_id = v;
        return self;
    }

    fn push_attr_l(mut self, v: u32) -> Self {
        self.3.attr_l = v;
        return self;
    }

    fn push_attr_s(mut self, v: u32) -> Self {
        self.3.attr_s = v;
        return self;
    }

    fn push_attr_sk(mut self, v: u32) -> Self {
        self.3.attr_sk = v;
        return self;
    }

    fn start_x(self) -> Self::X {
        return (self.0, self.1, self.2, self.3, ItemExtra::default());
    }
}

impl<'a> InventoryItemExtraDataSink for InventoryItemExtraStoreDataSink<'a> {
    type E = InventoryItemStoreDataSink<'a>;

    fn end_x(self) -> Self::E {
        let mut item = self.3;
        item.x = Some(self.4);
        return (self.0, self.1, self.2, item);
    }

    fn push_attr_b(mut self, v: String) -> Self {
        self.4.attr_b = v;
        return self;
    }

    fn push_attr_ma(mut self, v: String) -> Self {
        self.4.attr_ma = v;
        return self;
    }

    fn push_attr_ub(mut self, v: String) -> Self {
        self.4.attr_ub = v;
        return self;
    }

    fn push_attr_ud(mut self, v: String) -> Self {
        self.4.attr_ud = v;
        return self;
    }

    fn push_attr_ui(mut self, v: String) -> Self {
        self.4.attr_ui = v;
        return self;
    }

    fn push_attr_um(mut self, v: String) -> Self {
        self.4.attr_um = v;
        return self;
    }

    fn push_attr_un(mut self, v: String) -> Self {
        self.4.attr_ub = v;
        return self;
    }

    fn push_attr_uo(mut self, v: String) -> Self {
        self.4.attr_uo = v;
        return self;
    }

    fn push_attr_up(mut self, v: String) -> Self {
        self.4.attr_up = v;
        return self;
    }
}

impl<'a> MinifigDataSink for MinifigStoreDataSink<'a> {
    type E = ObjectStoreDataSink<'a>;

    fn end_mf(self) -> Self::E {
        let obj = self.0;
        obj.mf = Some(self.1);
        return obj;
    }

    fn push_attr_cd(mut self, v: u32) -> Self {
        self.1.attr_cd = v;
        return self;
    }

    fn push_attr_es(mut self, v: u32) -> Self {
        self.1.attr_es = v;
        return self;
    }

    fn push_attr_ess(mut self, v: u32) -> Self {
        self.1.attr_ess = v;
        return self;
    }

    fn push_attr_hc(mut self, v: u32) -> Self {
        self.1.attr_hc = v;
        return self;
    }

    fn push_attr_hd(mut self, v: u32) -> Self {
        self.1.attr_hd = v;
        return self;
    }

    fn push_attr_hdc(mut self, v: u32) -> Self {
        self.1.attr_hdc = v;
        return self;
    }

    fn push_attr_hs(mut self, v: u32) -> Self {
        self.1.attr_hs = v;
        return self;
    }

    fn push_attr_l(mut self, v: u32) -> Self {
        self.1.attr_l = v;
        return self;
    }

    fn push_attr_lh(mut self, v: u32) -> Self {
        self.1.attr_lh = v;
        return self;
    }

    fn push_attr_ms(mut self, v: u32) -> Self {
        self.1.attr_ms = v;
        return self;
    }

    fn push_attr_rh(mut self, v: u32) -> Self {
        self.1.attr_rh = v;
        return self;
    }

    fn push_attr_t(mut self, v: u32) -> Self {
        self.1.attr_t = v;
        return self;
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        let comp_obj = Object{
            dest: Some(Destructible{
                attr_d: Some(true),
                attr_im: Some(10),
                ..Destructible::default()
            }),
            inv: Some(Inventory{
                bag: vec![Bag{attr_t: 2, attr_m: 3}, Bag{attr_t: 4, attr_m: 8}],
                grps: vec![Group{attr_n: String::from("Name"), ..Group::default()}],
                items: Items{children: vec![
                    ItemBag{attr_t: 2, children: vec![
                        Item{attr_l: 1234, ..Item::default()},
                        Item{attr_l: 5678, ..Item::default()}
                    ]}
                ], ..Items::default()},
                ..Inventory::default()
            }),
            mf: Some(Minifig{
                attr_hs: 4,
                attr_cd: 10,
                ..Minifig::default()
            }),
            ..Object::default()
        };

        let mut test_obj = Object::default();
        let sink = &mut test_obj;
        sink.start_dest()
                .push_attr_d(true)
                .push_attr_im(10)
            .end_dest()
            .start_inv()
                .start_bag()
                    .start_b()
                        .push_attr_t(2)
                        .push_attr_m(3)
                    .end_b()
                    .start_b()
                        .push_attr_t(4)
                        .push_attr_m(8)
                    .end_b()
                .end_bag()
                .start_grps()
                    .start_grp()
                        .push_attr_n(String::from("Name"))
                    .end_grp()
                .end_grps()
                .start_items()
                    .start_in()
                        .push_attr_t(2)
                        .start_i().push_attr_l(1234).end_i()
                        .start_i().push_attr_l(5678).end_i()
                    .end_in()
                .end_items()
            .end_inv()
            .start_mf()
                .push_attr_hs(4)
                .push_attr_cd(10)
            .end_mf();

        assert_eq!(comp_obj, test_obj);
    }
}
