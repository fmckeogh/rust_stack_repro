#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use u_get_HCR_EL2_Type_E2H::*;
use HaveAArch32EL::*;
use HaveSecureEL2Ext::*;
use HaveAArch64::*;
use u__UNKNOWN_boolean::*;
use u_get_HCR_EL2_Type_RW::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_SCR_EL3_Type_EEL2::*;
use HaveVirtHostExt::*;
use u_get_SCR_EL3_Type_RW::*;
use common::*;
pub fn ELStateUsingAArch32K<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    secure: bool,
) -> ProductType8b847afc727d5818 {
    #[derive(Default)]
    struct FunctionState {
        gs_1748: bool,
        known: bool,
        gs_1754: bool,
        aarch32_below_el3: bool,
        gs_1761: bool,
        gs_1763: bool,
        return_value: ProductType8b847afc727d5818,
        aarch32_at_el1: bool,
        gs_1762: bool,
        gs_1743: bool,
        gs_1749: bool,
        gs_1750: bool,
        aarch32: bool,
        gs_1755: bool,
        gs_1757: bool,
        gs_1747: bool,
        gs_1764: bool,
        gs_1751: bool,
        gs_1758: bool,
        gs_1756: bool,
        gs_1753: bool,
        gs_1752: bool,
        el: u8,
        secure: bool,
    }
    let fn_state = FunctionState {
        el,
        secure,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_0_0: read-var el:u8
        let s_0_0: u8 = fn_state.el;
        // D s_0_1: call HaveAArch32EL(s_0_0)
        let s_0_1: bool = HaveAArch32EL(state, tracer, s_0_0);
        // D s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b66 b1
        if s_0_2 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_1_0: read-var secure:u8
        let s_1_0: bool = fn_state.secure;
        // N s_1_1: branch s_1_0 b65 b2
        if s_1_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#1743 <= s_2_0
        fn_state.gs_1743 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_3_0: read-var gs#1743:u8
        let s_3_0: bool = fn_state.gs_1743;
        // N s_3_1: branch s_3_0 b64 b4
        if s_3_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call HaveAArch64(s_4_0)
        let s_4_1: bool = HaveAArch64(state, tracer, s_4_0);
        // S s_4_2: not s_4_1
        let s_4_2: bool = !s_4_1;
        // N s_4_3: branch s_4_2 b63 b5
        if s_4_2 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call __UNKNOWN_boolean(s_5_0)
        let s_5_1: bool = u__UNKNOWN_boolean(state, tracer, s_5_0);
        // D s_5_2: write-var aarch32 <= s_5_1
        fn_state.aarch32 = s_5_1;
        // C s_5_3: const #1u : u8
        let s_5_3: bool = true;
        // D s_5_4: write-var known <= s_5_3
        fn_state.known = s_5_3;
        // C s_5_5: const #424u : u32
        let s_5_5: u32 = 424;
        // D s_5_6: read-reg s_5_5:u8
        let s_5_6: u8 = {
            let value = state.read_register::<u8>(s_5_5 as isize);
            tracer.read_register(s_5_5 as isize, value);
            value
        };
        // C s_5_7: const #2u : u8
        let s_5_7: u8 = 2;
        // D s_5_8: cmp-lt s_5_6 s_5_7
        let s_5_8: bool = ((s_5_6) < (s_5_7));
        // N s_5_9: branch s_5_8 b56 b6
        if s_5_8 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#1749 <= s_6_0
        fn_state.gs_1749 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_7_0: read-var gs#1749:u8
        let s_7_0: bool = fn_state.gs_1749;
        // N s_7_1: branch s_7_0 b55 b8
        if s_7_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#1750 <= s_8_0
        fn_state.gs_1750 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_9_0: read-var gs#1750:u8
        let s_9_0: bool = fn_state.gs_1750;
        // D s_9_1: write-var aarch32_below_el3 <= s_9_0
        fn_state.aarch32_below_el3 = s_9_0;
        // D s_9_2: read-var aarch32_below_el3:u8
        let s_9_2: bool = fn_state.aarch32_below_el3;
        // N s_9_3: branch s_9_2 b54 b10
        if s_9_2 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_10_0: const #432u : u32
        let s_10_0: u32 = 432;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #2u : u8
        let s_10_2: u8 = 2;
        // D s_10_3: cmp-lt s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) < (s_10_2));
        // N s_10_4: branch s_10_3 b47 b11
        if s_10_3 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#1753 <= s_11_0
        fn_state.gs_1753 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_12_0: read-var gs#1753:u8
        let s_12_0: bool = fn_state.gs_1753;
        // N s_12_1: branch s_12_0 b43 b13
        if s_12_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#1755 <= s_13_0
        fn_state.gs_1755 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_14_0: read-var gs#1755:u8
        let s_14_0: bool = fn_state.gs_1755;
        // N s_14_1: branch s_14_0 b42 b15
        if s_14_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#1756 <= s_15_0
        fn_state.gs_1756 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_16_0: read-var gs#1756:u8
        let s_16_0: bool = fn_state.gs_1756;
        // D s_16_1: write-var gs#1757 <= s_16_0
        fn_state.gs_1757 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_17_0: read-var gs#1757:u8
        let s_17_0: bool = fn_state.gs_1757;
        // D s_17_1: write-var aarch32_at_el1 <= s_17_0
        fn_state.aarch32_at_el1 = s_17_0;
        // D s_17_2: read-var el:u8
        let s_17_2: u8 = fn_state.el;
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // C s_17_4: const #448u : u32
        let s_17_4: u32 = 448;
        // D s_17_5: read-reg s_17_4:u8
        let s_17_5: u8 = {
            let value = state.read_register::<u8>(s_17_4 as isize);
            tracer.read_register(s_17_4 as isize, value);
            value
        };
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 2u16);
        // D s_17_7: cmp-eq s_17_3 s_17_6
        let s_17_7: bool = ((s_17_3) == (s_17_6));
        // N s_17_8: branch s_17_7 b41 b18
        if s_17_7 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#1758 <= s_18_0
        fn_state.gs_1758 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_19_0: read-var gs#1758:u8
        let s_19_0: bool = fn_state.gs_1758;
        // N s_19_1: branch s_19_0 b38 b20
        if s_19_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_20_0: read-var aarch32_below_el3:u8
        let s_20_0: bool = fn_state.aarch32_below_el3;
        // N s_20_1: branch s_20_0 b37 b21
        if s_20_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#1761 <= s_21_0
        fn_state.gs_1761 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_22_0: read-var gs#1761:u8
        let s_22_0: bool = fn_state.gs_1761;
        // N s_22_1: branch s_22_0 b36 b23
        if s_22_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_23_0: read-var aarch32_at_el1:u8
        let s_23_0: bool = fn_state.aarch32_at_el1;
        // N s_23_1: branch s_23_0 b32 b24
        if s_23_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#1763 <= s_24_0
        fn_state.gs_1763 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_25_0: read-var gs#1763:u8
        let s_25_0: bool = fn_state.gs_1763;
        // D s_25_1: write-var gs#1764 <= s_25_0
        fn_state.gs_1764 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_26_0: read-var gs#1764:u8
        let s_26_0: bool = fn_state.gs_1764;
        // D s_26_1: write-var aarch32 <= s_26_0
        fn_state.aarch32 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_27_0: read-var known:u8
        let s_27_0: bool = fn_state.known;
        // D s_27_1: not s_27_0
        let s_27_1: bool = !s_27_0;
        // N s_27_2: branch s_27_1 b31 b28
        if s_27_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // N s_28_0: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_29_0: read-var known:u8
        let s_29_0: bool = fn_state.known;
        // D s_29_1: read-var aarch32:u8
        let s_29_1: bool = fn_state.aarch32;
        // D s_29_2: create-product struct = ["s_29_0", "s_29_1"]
        let s_29_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_29_0,
            _1: s_29_1,
        };
        // D s_29_3: write-var return_value <= s_29_2
        fn_state.return_value = s_29_2;
        // N s_29_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_30_0: read-var return_value:struct
        let s_30_0: ProductType8b847afc727d5818 = fn_state.return_value;
        // N s_30_1: return s_30_0
        return s_30_0;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call __UNKNOWN_boolean(s_31_0)
        let s_31_1: bool = u__UNKNOWN_boolean(state, tracer, s_31_0);
        // D s_31_2: write-var aarch32 <= s_31_1
        fn_state.aarch32 = s_31_1;
        // N s_31_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_32_0: read-var el:u8
        let s_32_0: u8 = fn_state.el;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 2u16);
        // C s_32_2: const #440u : u32
        let s_32_2: u32 = 440;
        // D s_32_3: read-reg s_32_2:u8
        let s_32_3: u8 = {
            let value = state.read_register::<u8>(s_32_2 as isize);
            tracer.read_register(s_32_2 as isize, value);
            value
        };
        // D s_32_4: cast zx s_32_3 -> bv
        let s_32_4: Bits = Bits::new(s_32_3 as u128, 2u16);
        // D s_32_5: cmp-eq s_32_1 s_32_4
        let s_32_5: bool = ((s_32_1) == (s_32_4));
        // N s_32_6: branch s_32_5 b35 b33
        if s_32_5 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_33_0: read-var el:u8
        let s_33_0: u8 = fn_state.el;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 2u16);
        // C s_33_2: const #448u : u32
        let s_33_2: u32 = 448;
        // D s_33_3: read-reg s_33_2:u8
        let s_33_3: u8 = {
            let value = state.read_register::<u8>(s_33_2 as isize);
            tracer.read_register(s_33_2 as isize, value);
            value
        };
        // D s_33_4: cast zx s_33_3 -> bv
        let s_33_4: Bits = Bits::new(s_33_3 as u128, 2u16);
        // D s_33_5: cmp-eq s_33_1 s_33_4
        let s_33_5: bool = ((s_33_1) == (s_33_4));
        // D s_33_6: write-var gs#1762 <= s_33_5
        fn_state.gs_1762 = s_33_5;
        // N s_33_7: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_34_0: read-var gs#1762:u8
        let s_34_0: bool = fn_state.gs_1762;
        // D s_34_1: write-var gs#1763 <= s_34_0
        fn_state.gs_1763 = s_34_0;
        // N s_34_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#1762 <= s_35_0
        fn_state.gs_1762 = s_35_0;
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#1764 <= s_36_0
        fn_state.gs_1764 = s_36_0;
        // N s_36_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_37_0: read-var el:u8
        let s_37_0: u8 = fn_state.el;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 2u16);
        // C s_37_2: const #424u : u32
        let s_37_2: u32 = 424;
        // D s_37_3: read-reg s_37_2:u8
        let s_37_3: u8 = {
            let value = state.read_register::<u8>(s_37_2 as isize);
            tracer.read_register(s_37_2 as isize, value);
            value
        };
        // D s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 2u16);
        // D s_37_5: cmp-ne s_37_1 s_37_4
        let s_37_5: bool = ((s_37_1) != (s_37_4));
        // D s_37_6: write-var gs#1761 <= s_37_5
        fn_state.gs_1761 = s_37_5;
        // N s_37_7: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_38_0: const #16975u : u32
        let s_38_0: u32 = 16975;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: cast zx s_38_1 -> bv
        let s_38_2: Bits = Bits::new(s_38_1 as u128, 2u16);
        // C s_38_3: const #448u : u32
        let s_38_3: u32 = 448;
        // D s_38_4: read-reg s_38_3:u8
        let s_38_4: u8 = {
            let value = state.read_register::<u8>(s_38_3 as isize);
            tracer.read_register(s_38_3 as isize, value);
            value
        };
        // D s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 2u16);
        // D s_38_6: cmp-eq s_38_2 s_38_5
        let s_38_6: bool = ((s_38_2) == (s_38_5));
        // N s_38_7: branch s_38_6 b40 b39
        if s_38_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var known <= s_39_0
        fn_state.known = s_39_0;
        // N s_39_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_40_0: const #16999u : u32
        let s_40_0: u32 = 16999;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: bool = {
            let value = state.read_register::<bool>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 1u16);
        // C s_40_3: const #1u : u8
        let s_40_3: bool = true;
        // C s_40_4: cast zx s_40_3 -> bv
        let s_40_4: Bits = Bits::new(s_40_3 as u128, 1u16);
        // D s_40_5: cmp-eq s_40_2 s_40_4
        let s_40_5: bool = ((s_40_2) == (s_40_4));
        // D s_40_6: write-var aarch32 <= s_40_5
        fn_state.aarch32 = s_40_5;
        // N s_40_7: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_41_0: read-var aarch32_at_el1:u8
        let s_41_0: bool = fn_state.aarch32_at_el1;
        // D s_41_1: not s_41_0
        let s_41_1: bool = !s_41_0;
        // D s_41_2: write-var gs#1758 <= s_41_1
        fn_state.gs_1758 = s_41_1;
        // N s_41_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_42_0: const #102552u : u32
        let s_42_0: u32 = 102552;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_HCR_EL2_Type_RW(s_42_1)
        let s_42_2: bool = u_get_HCR_EL2_Type_RW(state, tracer, s_42_1);
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #0u : u8
        let s_42_4: bool = false;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#1756 <= s_42_6
        fn_state.gs_1756 = s_42_6;
        // N s_42_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call HaveVirtHostExt(s_43_0)
        let s_43_1: bool = HaveVirtHostExt(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b46 b44
        if s_43_1 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#1754 <= s_44_0
        fn_state.gs_1754 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_45_0: read-var gs#1754:u8
        let s_45_0: bool = fn_state.gs_1754;
        // D s_45_1: not s_45_0
        let s_45_1: bool = !s_45_0;
        // D s_45_2: write-var gs#1755 <= s_45_1
        fn_state.gs_1755 = s_45_1;
        // N s_45_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_46_0: const #102552u : u32
        let s_46_0: u32 = 102552;
        // D s_46_1: read-reg s_46_0:struct
        let s_46_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call _get_HCR_EL2_Type_E2H(s_46_1)
        let s_46_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_46_1);
        // C s_46_3: const #102552u : u32
        let s_46_3: u32 = 102552;
        // D s_46_4: read-reg s_46_3:struct
        let s_46_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_3 as isize);
            tracer.read_register(s_46_3 as isize, value);
            value
        };
        // D s_46_5: call _get_HCR_EL2_Type_TGE(s_46_4)
        let s_46_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_46_4);
        // D s_46_6: cast zx s_46_2 -> bv
        let s_46_6: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_7: cast zx s_46_5 -> bv
        let s_46_7: Bits = Bits::new(s_46_5 as u128, 1u16);
        // D s_46_8: cast reint s_46_6 -> u128
        let s_46_8: u128 = (s_46_6.value() as u128);
        // D s_46_9: size-of s_46_6
        let s_46_9: u16 = s_46_6.length();
        // D s_46_10: cast reint s_46_7 -> u128
        let s_46_10: u128 = (s_46_7.value() as u128);
        // D s_46_11: size-of s_46_7
        let s_46_11: u16 = s_46_7.length();
        // D s_46_12: lsl s_46_8 s_46_11
        let s_46_12: u128 = s_46_8 << s_46_11;
        // D s_46_13: or s_46_12 s_46_10
        let s_46_13: u128 = ((s_46_12) | (s_46_10));
        // D s_46_14: add s_46_9 s_46_11
        let s_46_14: u16 = (s_46_9 + s_46_11);
        // D s_46_15: create-bits s_46_13 s_46_14
        let s_46_15: Bits = Bits::new(s_46_13, s_46_14);
        // D s_46_16: cast reint s_46_15 -> u8
        let s_46_16: u8 = (s_46_15.value() as u8);
        // D s_46_17: cast zx s_46_16 -> bv
        let s_46_17: Bits = Bits::new(s_46_16 as u128, 2u16);
        // C s_46_18: const #3u : u8
        let s_46_18: u8 = 3;
        // C s_46_19: cast zx s_46_18 -> bv
        let s_46_19: Bits = Bits::new(s_46_18 as u128, 2u16);
        // D s_46_20: cmp-eq s_46_17 s_46_19
        let s_46_20: bool = ((s_46_17) == (s_46_19));
        // D s_46_21: write-var gs#1754 <= s_46_20
        fn_state.gs_1754 = s_46_20;
        // N s_46_22: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_47_0: read-var secure:u8
        let s_47_0: bool = fn_state.secure;
        // D s_47_1: not s_47_0
        let s_47_1: bool = !s_47_0;
        // N s_47_2: branch s_47_1 b53 b48
        if s_47_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call HaveSecureEL2Ext(s_48_0)
        let s_48_1: bool = HaveSecureEL2Ext(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b52 b49
        if s_48_1 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#1751 <= s_49_0
        fn_state.gs_1751 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_50_0: read-var gs#1751:u8
        let s_50_0: bool = fn_state.gs_1751;
        // D s_50_1: write-var gs#1752 <= s_50_0
        fn_state.gs_1752 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_51_0: read-var gs#1752:u8
        let s_51_0: bool = fn_state.gs_1752;
        // D s_51_1: write-var gs#1753 <= s_51_0
        fn_state.gs_1753 = s_51_0;
        // N s_51_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_52_0: const #90704u : u32
        let s_52_0: u32 = 90704;
        // D s_52_1: read-reg s_52_0:struct
        let s_52_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call _get_SCR_EL3_Type_EEL2(s_52_1)
        let s_52_2: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_52_1);
        // D s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // C s_52_4: const #1u : u8
        let s_52_4: bool = true;
        // C s_52_5: cast zx s_52_4 -> bv
        let s_52_5: Bits = Bits::new(s_52_4 as u128, 1u16);
        // D s_52_6: cmp-eq s_52_3 s_52_5
        let s_52_6: bool = ((s_52_3) == (s_52_5));
        // D s_52_7: write-var gs#1751 <= s_52_6
        fn_state.gs_1751 = s_52_6;
        // N s_52_8: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#1752 <= s_53_0
        fn_state.gs_1752 = s_53_0;
        // N s_53_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#1757 <= s_54_0
        fn_state.gs_1757 = s_54_0;
        // N s_54_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_55_0: const #90704u : u32
        let s_55_0: u32 = 90704;
        // D s_55_1: read-reg s_55_0:struct
        let s_55_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call _get_SCR_EL3_Type_RW(s_55_1)
        let s_55_2: bool = u_get_SCR_EL3_Type_RW(state, tracer, s_55_1);
        // D s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #0u : u8
        let s_55_4: bool = false;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // D s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // D s_55_7: write-var gs#1750 <= s_55_6
        fn_state.gs_1750 = s_55_6;
        // N s_55_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_56_0: read-var secure:u8
        let s_56_0: bool = fn_state.secure;
        // D s_56_1: not s_56_0
        let s_56_1: bool = !s_56_0;
        // N s_56_2: branch s_56_1 b62 b57
        if s_56_1 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call HaveSecureEL2Ext(s_57_0)
        let s_57_1: bool = HaveSecureEL2Ext(state, tracer, s_57_0);
        // S s_57_2: not s_57_1
        let s_57_2: bool = !s_57_1;
        // D s_57_3: write-var gs#1747 <= s_57_2
        fn_state.gs_1747 = s_57_2;
        // N s_57_4: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_58_0: read-var gs#1747:u8
        let s_58_0: bool = fn_state.gs_1747;
        // N s_58_1: branch s_58_0 b61 b59
        if s_58_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_59_0: const #90704u : u32
        let s_59_0: u32 = 90704;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_SCR_EL3_Type_EEL2(s_59_1)
        let s_59_2: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_59_1);
        // D s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // C s_59_4: const #0u : u8
        let s_59_4: bool = false;
        // C s_59_5: cast zx s_59_4 -> bv
        let s_59_5: Bits = Bits::new(s_59_4 as u128, 1u16);
        // D s_59_6: cmp-eq s_59_3 s_59_5
        let s_59_6: bool = ((s_59_3) == (s_59_5));
        // D s_59_7: write-var gs#1748 <= s_59_6
        fn_state.gs_1748 = s_59_6;
        // N s_59_8: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_60_0: read-var gs#1748:u8
        let s_60_0: bool = fn_state.gs_1748;
        // D s_60_1: write-var gs#1749 <= s_60_0
        fn_state.gs_1749 = s_60_0;
        // N s_60_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#1748 <= s_61_0
        fn_state.gs_1748 = s_61_0;
        // N s_61_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#1747 <= s_62_0
        fn_state.gs_1747 = s_62_0;
        // N s_62_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // C s_63_1: const #1u : u8
        let s_63_1: bool = true;
        // D s_63_2: create-product struct = ["s_63_0", "s_63_1"]
        let s_63_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_63_0,
            _1: s_63_1,
        };
        // D s_63_3: write-var return_value <= s_63_2
        fn_state.return_value = s_63_2;
        // N s_63_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // C s_64_1: const #0u : u8
        let s_64_1: bool = false;
        // D s_64_2: create-product struct = ["s_64_0", "s_64_1"]
        let s_64_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_64_0,
            _1: s_64_1,
        };
        // D s_64_3: write-var return_value <= s_64_2
        fn_state.return_value = s_64_2;
        // N s_64_4: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // D s_65_0: read-var el:u8
        let s_65_0: u8 = fn_state.el;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 2u16);
        // C s_65_2: const #432u : u32
        let s_65_2: u32 = 432;
        // D s_65_3: read-reg s_65_2:u8
        let s_65_3: u8 = {
            let value = state.read_register::<u8>(s_65_2 as isize);
            tracer.read_register(s_65_2 as isize, value);
            value
        };
        // D s_65_4: cast zx s_65_3 -> bv
        let s_65_4: Bits = Bits::new(s_65_3 as u128, 2u16);
        // D s_65_5: cmp-eq s_65_1 s_65_4
        let s_65_5: bool = ((s_65_1) == (s_65_4));
        // D s_65_6: write-var gs#1743 <= s_65_5
        fn_state.gs_1743 = s_65_5;
        // N s_65_7: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType8b847afc727d5818 {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // C s_66_1: const #0u : u8
        let s_66_1: bool = false;
        // D s_66_2: create-product struct = ["s_66_0", "s_66_1"]
        let s_66_2: ProductType8b847afc727d5818 = ProductType8b847afc727d5818 {
            _0: s_66_0,
            _1: s_66_1,
        };
        // D s_66_3: write-var return_value <= s_66_2
        fn_state.return_value = s_66_2;
        // N s_66_4: jump b30
        return block_30(state, tracer, fn_state);
    }
}
