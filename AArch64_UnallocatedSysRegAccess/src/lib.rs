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
use u_get_SCTLR_EL2_Type_TIDCP::*;
use IsInHost::*;
use HaveFGTExt::*;
use u__IMPDEF_boolean::*;
use HaveIDSExt::*;
use AArch64_SystemAccessTrap::*;
use Zeros::*;
use X_set::*;
use u_get_SCTLR_EL1_Type_TIDCP::*;
use HaveFeatTIDCP1::*;
use ELUsingAArch32::*;
use u_get_HCR_EL2_Type_TID3::*;
use u_get_HCR_EL2_Type_TIDCP::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn AArch64_UnallocatedSysRegAccess<T: Tracer>(
    state: &mut State,
    tracer: &T,
    op0: u8,
    op1: u8,
    crn: u8,
    op2: u8,
    crm: u8,
    read: bool,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_141846: bool,
        gs_141787: bool,
        gs_141815: bool,
        gs_141795: bool,
        gs_141782: bool,
        gs_141783: bool,
        gs_141779: bool,
        gs_141763: bool,
        gs_141771: bool,
        gs_141793: bool,
        gs_141781: bool,
        gs_141838: bool,
        gs_141874: bool,
        rcs_el0_trapshadow_1011: bool,
        gs_141858: bool,
        gs_141842: bool,
        gs_141871: bool,
        ec_val: i128,
        gs_141821: bool,
        gs_141762: bool,
        gs_141841: bool,
        gs_141844: bool,
        gs_141832: bool,
        gs_141831: bool,
        gs_141780: bool,
        gs_141833: bool,
        gs_141873: bool,
        in_feature_space: bool,
        gs_141853: bool,
        gs_141776: bool,
        gs_141777: bool,
        gs_141761: bool,
        gs_141823: bool,
        gs_141828: bool,
        gs_141790: bool,
        gs_141845: bool,
        gs_141764: bool,
        gs_141788: bool,
        gs_141859: bool,
        gs_141789: bool,
        gs_141862: bool,
        gs_141765: bool,
        gs_141766: bool,
        gs_141775: bool,
        gs_141791: bool,
        gs_141785: bool,
        gs_141866: bool,
        gs_141869: bool,
        gs_141794: bool,
        gs_141865: bool,
        gs_141784: bool,
        gs_141786: bool,
        gs_141847: bool,
        gs_141822: bool,
        gs_141872: bool,
        tid3_trap: bool,
        b__0: u8,
        gs_141772: bool,
        gs_141843: bool,
        gs_141792: bool,
        gs_141848: bool,
        ec_valshadow_1009: i128,
        gs_141773: bool,
        gs_141768: bool,
        gs_141769: bool,
        gs_141778: bool,
        gs_141774: bool,
        op0: u8,
        op1: u8,
        crn: u8,
        op2: u8,
        crm: u8,
        read: bool,
        t: i128,
    }
    let fn_state = FunctionState {
        op0,
        op1,
        crn,
        op2,
        crm,
        read,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u8
        let s_0_0: bool = false;
        // D s_0_1: write-var tid3_trap <= s_0_0
        fn_state.tid3_trap = s_0_0;
        // C s_0_2: const #16975u : u32
        let s_0_2: u32 = 16975;
        // D s_0_3: read-reg s_0_2:u8
        let s_0_3: u8 = {
            let value = state.read_register::<u8>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #440u : u32
        let s_0_5: u32 = 440;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // N s_0_9: branch s_0_8 b258 b1
        if s_0_8 {
            return block_258(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#141761 <= s_1_0
        fn_state.gs_141761 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#141761:u8
        let s_2_0: bool = fn_state.gs_141761;
        // N s_2_1: branch s_2_0 b257 b3
        if s_2_0 {
            return block_257(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#141762 <= s_3_0
        fn_state.gs_141762 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#141762:u8
        let s_4_0: bool = fn_state.gs_141762;
        // N s_4_1: branch s_4_0 b256 b5
        if s_4_0 {
            return block_256(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#141763 <= s_5_0
        fn_state.gs_141763 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#141763:u8
        let s_6_0: bool = fn_state.gs_141763;
        // N s_6_1: branch s_6_0 b252 b7
        if s_6_0 {
            return block_252(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #16975u : u32
        let s_8_0: u32 = 16975;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 2u16);
        // C s_8_3: const #440u : u32
        let s_8_3: u32 = 440;
        // D s_8_4: read-reg s_8_3:u8
        let s_8_4: u8 = {
            let value = state.read_register::<u8>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 2u16);
        // D s_8_6: cmp-eq s_8_2 s_8_5
        let s_8_6: bool = ((s_8_2) == (s_8_5));
        // N s_8_7: branch s_8_6 b251 b9
        if s_8_6 {
            return block_251(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16975u : u32
        let s_9_0: u32 = 16975;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 2u16);
        // C s_9_3: const #432u : u32
        let s_9_3: u32 = 432;
        // D s_9_4: read-reg s_9_3:u8
        let s_9_4: u8 = {
            let value = state.read_register::<u8>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 2u16);
        // D s_9_6: cmp-eq s_9_2 s_9_5
        let s_9_6: bool = ((s_9_2) == (s_9_5));
        // N s_9_7: branch s_9_6 b250 b10
        if s_9_6 {
            return block_250(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #16975u : u32
        let s_10_0: u32 = 16975;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 2u16);
        // C s_10_3: const #424u : u32
        let s_10_3: u32 = 424;
        // D s_10_4: read-reg s_10_3:u8
        let s_10_4: u8 = {
            let value = state.read_register::<u8>(s_10_3 as isize);
            tracer.read_register(s_10_3 as isize, value);
            value
        };
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 2u16);
        // D s_10_6: cmp-eq s_10_2 s_10_5
        let s_10_6: bool = ((s_10_2) == (s_10_5));
        // D s_10_7: write-var gs#141764 <= s_10_6
        fn_state.gs_141764 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#141764:u8
        let s_11_0: bool = fn_state.gs_141764;
        // D s_11_1: write-var gs#141765 <= s_11_0
        fn_state.gs_141765 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#141765:u8
        let s_12_0: bool = fn_state.gs_141765;
        // N s_12_1: branch s_12_0 b249 b13
        if s_12_0 {
            return block_249(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#141766 <= s_13_0
        fn_state.gs_141766 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#141766:u8
        let s_14_0: bool = fn_state.gs_141766;
        // N s_14_1: branch s_14_0 b140 b15
        if s_14_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var op0:u8
        let s_16_0: u8 = fn_state.op0;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 2u16);
        // C s_16_2: const #3u : u8
        let s_16_2: u8 = 3;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 2u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // N s_16_5: branch s_16_4 b134 b17
        if s_16_4 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#141821 <= s_17_0
        fn_state.gs_141821 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#141821:u8
        let s_18_0: bool = fn_state.gs_141821;
        // N s_18_1: branch s_18_0 b133 b19
        if s_18_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#141822 <= s_19_0
        fn_state.gs_141822 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#141822:u8
        let s_20_0: bool = fn_state.gs_141822;
        // N s_20_1: branch s_20_0 b129 b21
        if s_20_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#141828 <= s_21_0
        fn_state.gs_141828 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#141828:u8
        let s_22_0: bool = fn_state.gs_141828;
        // D s_22_1: write-var in_feature_space <= s_22_0
        fn_state.in_feature_space = s_22_0;
        // C s_22_2: const #0u : u8
        let s_22_2: u8 = 0;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 4u16);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (s_22_3.value() as i128);
        // D s_22_5: write-var ec_val <= s_22_4
        fn_state.ec_val = s_22_4;
        // C s_22_6: const #() : ()
        let s_22_6: () = ();
        // S s_22_7: call HaveIDSExt(s_22_6)
        let s_22_7: bool = HaveIDSExt(state, tracer, s_22_6);
        // N s_22_8: branch s_22_7 b128 b23
        if s_22_7 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ec_val:i
        let s_24_0: i128 = fn_state.ec_val;
        // D s_24_1: write-var ec_valshadow#1009 <= s_24_0
        fn_state.ec_valshadow_1009 = s_24_0;
        // D s_24_2: read-var in_feature_space:u8
        let s_24_2: bool = fn_state.in_feature_space;
        // N s_24_3: branch s_24_2 b127 b25
        if s_24_2 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#141831 <= s_25_0
        fn_state.gs_141831 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#141831:u8
        let s_26_0: bool = fn_state.gs_141831;
        // N s_26_1: branch s_26_0 b102 b27
        if s_26_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HaveFeatTIDCP1(s_28_0)
        let s_28_1: bool = HaveFeatTIDCP1(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b101 b29
        if s_28_1 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#141832 <= s_29_0
        fn_state.gs_141832 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#141832:u8
        let s_30_0: bool = fn_state.gs_141832;
        // N s_30_1: branch s_30_0 b94 b31
        if s_30_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#141841 <= s_31_0
        fn_state.gs_141841 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#141841:u8
        let s_32_0: bool = fn_state.gs_141841;
        // N s_32_1: branch s_32_0 b76 b33
        if s_32_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #16975u : u32
        let s_34_0: u32 = 16975;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: cast zx s_34_1 -> bv
        let s_34_2: Bits = Bits::new(s_34_1 as u128, 2u16);
        // C s_34_3: const #448u : u32
        let s_34_3: u32 = 448;
        // D s_34_4: read-reg s_34_3:u8
        let s_34_4: u8 = {
            let value = state.read_register::<u8>(s_34_3 as isize);
            tracer.read_register(s_34_3 as isize, value);
            value
        };
        // D s_34_5: cast zx s_34_4 -> bv
        let s_34_5: Bits = Bits::new(s_34_4 as u128, 2u16);
        // D s_34_6: cmp-eq s_34_2 s_34_5
        let s_34_6: bool = ((s_34_2) == (s_34_5));
        // N s_34_7: branch s_34_6 b75 b35
        if s_34_6 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #16975u : u32
        let s_35_0: u32 = 16975;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: cast zx s_35_1 -> bv
        let s_35_2: Bits = Bits::new(s_35_1 as u128, 2u16);
        // C s_35_3: const #440u : u32
        let s_35_3: u32 = 440;
        // D s_35_4: read-reg s_35_3:u8
        let s_35_4: u8 = {
            let value = state.read_register::<u8>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // D s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 2u16);
        // D s_35_6: cmp-eq s_35_2 s_35_5
        let s_35_6: bool = ((s_35_2) == (s_35_5));
        // D s_35_7: write-var gs#141842 <= s_35_6
        fn_state.gs_141842 = s_35_6;
        // N s_35_8: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#141842:u8
        let s_36_0: bool = fn_state.gs_141842;
        // N s_36_1: branch s_36_0 b74 b37
        if s_36_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#141843 <= s_37_0
        fn_state.gs_141843 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#141843:u8
        let s_38_0: bool = fn_state.gs_141843;
        // N s_38_1: branch s_38_0 b73 b39
        if s_38_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#141844 <= s_39_0
        fn_state.gs_141844 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#141844:u8
        let s_40_0: bool = fn_state.gs_141844;
        // N s_40_1: branch s_40_0 b69 b41
        if s_40_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#141846 <= s_41_0
        fn_state.gs_141846 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#141846:u8
        let s_42_0: bool = fn_state.gs_141846;
        // N s_42_1: branch s_42_0 b65 b43
        if s_42_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#141848 <= s_43_0
        fn_state.gs_141848 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#141848:u8
        let s_44_0: bool = fn_state.gs_141848;
        // N s_44_1: branch s_44_0 b58 b45
        if s_44_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0s : i
        let s_46_0: i128 = 0;
        // C s_46_1: const #2s : i
        let s_46_1: i128 = 2;
        // D s_46_2: read-var op0:u8
        let s_46_2: u8 = fn_state.op0;
        // D s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 2u16);
        // D s_46_4: bit-extract s_46_3 s_46_0 s_46_1
        let s_46_4: Bits = (Bits::new(
            ((s_46_3) >> (s_46_0)).value(),
            u16::try_from(s_46_1).unwrap(),
        ));
        // D s_46_5: cast reint s_46_4 -> u8
        let s_46_5: u8 = (s_46_4.value() as u8);
        // D s_46_6: cast zx s_46_5 -> bv
        let s_46_6: Bits = Bits::new(s_46_5 as u128, 2u16);
        // C s_46_7: const #1u : u8
        let s_46_7: u8 = 1;
        // C s_46_8: cast zx s_46_7 -> bv
        let s_46_8: Bits = Bits::new(s_46_7 as u128, 2u16);
        // D s_46_9: cmp-eq s_46_6 s_46_8
        let s_46_9: bool = ((s_46_6) == (s_46_8));
        // N s_46_10: branch s_46_9 b57 b47
        if s_46_9 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0s : i
        let s_47_0: i128 = 0;
        // C s_47_1: const #2s : i
        let s_47_1: i128 = 2;
        // D s_47_2: read-var op0:u8
        let s_47_2: u8 = fn_state.op0;
        // D s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 2u16);
        // D s_47_4: bit-extract s_47_3 s_47_0 s_47_1
        let s_47_4: Bits = (Bits::new(
            ((s_47_3) >> (s_47_0)).value(),
            u16::try_from(s_47_1).unwrap(),
        ));
        // D s_47_5: cast reint s_47_4 -> u8
        let s_47_5: u8 = (s_47_4.value() as u8);
        // D s_47_6: cast zx s_47_5 -> bv
        let s_47_6: Bits = Bits::new(s_47_5 as u128, 2u16);
        // C s_47_7: const #3u : u8
        let s_47_7: u8 = 3;
        // C s_47_8: cast zx s_47_7 -> bv
        let s_47_8: Bits = Bits::new(s_47_7 as u128, 2u16);
        // D s_47_9: cmp-eq s_47_6 s_47_8
        let s_47_9: bool = ((s_47_6) == (s_47_8));
        // D s_47_10: write-var gs#141853 <= s_47_9
        fn_state.gs_141853 = s_47_9;
        // N s_47_11: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#141853:u8
        let s_48_0: bool = fn_state.gs_141853;
        // N s_48_1: branch s_48_0 b53 b49
        if s_48_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#141859 <= s_49_0
        fn_state.gs_141859 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#141859:u8
        let s_50_0: bool = fn_state.gs_141859;
        // N s_50_1: branch s_50_0 b52 b51
        if s_50_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: panic
        panic!("{:?}", ());
        // N s_51_1: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: panic
        panic!("{:?}", ());
        // N s_52_1: return
        return;
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0s : i
        let s_53_0: i128 = 0;
        // C s_53_1: const #4s : i
        let s_53_1: i128 = 4;
        // D s_53_2: read-var crn:u8
        let s_53_2: u8 = fn_state.crn;
        // D s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 4u16);
        // D s_53_4: bit-extract s_53_3 s_53_0 s_53_1
        let s_53_4: Bits = (Bits::new(
            ((s_53_3) >> (s_53_0)).value(),
            u16::try_from(s_53_1).unwrap(),
        ));
        // D s_53_5: cast reint s_53_4 -> u8
        let s_53_5: u8 = (s_53_4.value() as u8);
        // D s_53_6: cast zx s_53_5 -> bv
        let s_53_6: Bits = Bits::new(s_53_5 as u128, 4u16);
        // C s_53_7: const #11u : u8
        let s_53_7: u8 = 11;
        // C s_53_8: cast zx s_53_7 -> bv
        let s_53_8: Bits = Bits::new(s_53_7 as u128, 4u16);
        // D s_53_9: cmp-eq s_53_6 s_53_8
        let s_53_9: bool = ((s_53_6) == (s_53_8));
        // N s_53_10: branch s_53_9 b56 b54
        if s_53_9 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0s : i
        let s_54_0: i128 = 0;
        // C s_54_1: const #4s : i
        let s_54_1: i128 = 4;
        // D s_54_2: read-var crn:u8
        let s_54_2: u8 = fn_state.crn;
        // D s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 4u16);
        // D s_54_4: bit-extract s_54_3 s_54_0 s_54_1
        let s_54_4: Bits = (Bits::new(
            ((s_54_3) >> (s_54_0)).value(),
            u16::try_from(s_54_1).unwrap(),
        ));
        // D s_54_5: cast reint s_54_4 -> u8
        let s_54_5: u8 = (s_54_4.value() as u8);
        // D s_54_6: cast zx s_54_5 -> bv
        let s_54_6: Bits = Bits::new(s_54_5 as u128, 4u16);
        // C s_54_7: const #15u : u8
        let s_54_7: u8 = 15;
        // C s_54_8: cast zx s_54_7 -> bv
        let s_54_8: Bits = Bits::new(s_54_7 as u128, 4u16);
        // D s_54_9: cmp-eq s_54_6 s_54_8
        let s_54_9: bool = ((s_54_6) == (s_54_8));
        // D s_54_10: write-var gs#141858 <= s_54_9
        fn_state.gs_141858 = s_54_9;
        // N s_54_11: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#141858:u8
        let s_55_0: bool = fn_state.gs_141858;
        // D s_55_1: write-var gs#141859 <= s_55_0
        fn_state.gs_141859 = s_55_0;
        // N s_55_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#141858 <= s_56_0
        fn_state.gs_141858 = s_56_0;
        // N s_56_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #1u : u8
        let s_57_0: bool = true;
        // D s_57_1: write-var gs#141853 <= s_57_0
        fn_state.gs_141853 = s_57_0;
        // N s_57_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #"Reserved Control Space EL0 Trapped" : str
        let s_58_0: &'static str = "Reserved Control Space EL0 Trapped";
        // S s_58_1: call __IMPDEF_boolean(s_58_0)
        let s_58_1: bool = u__IMPDEF_boolean(state, tracer, s_58_0);
        // D s_58_2: write-var rcs_el0_trapshadow#1011 <= s_58_1
        fn_state.rcs_el0_trapshadow_1011 = s_58_1;
        // C s_58_3: const #16975u : u32
        let s_58_3: u32 = 16975;
        // D s_58_4: read-reg s_58_3:u8
        let s_58_4: u8 = {
            let value = state.read_register::<u8>(s_58_3 as isize);
            tracer.read_register(s_58_3 as isize, value);
            value
        };
        // D s_58_5: cast zx s_58_4 -> bv
        let s_58_5: Bits = Bits::new(s_58_4 as u128, 2u16);
        // C s_58_6: const #440u : u32
        let s_58_6: u32 = 440;
        // D s_58_7: read-reg s_58_6:u8
        let s_58_7: u8 = {
            let value = state.read_register::<u8>(s_58_6 as isize);
            tracer.read_register(s_58_6 as isize, value);
            value
        };
        // D s_58_8: cast zx s_58_7 -> bv
        let s_58_8: Bits = Bits::new(s_58_7 as u128, 2u16);
        // D s_58_9: cmp-eq s_58_5 s_58_8
        let s_58_9: bool = ((s_58_5) == (s_58_8));
        // N s_58_10: branch s_58_9 b64 b59
        if s_58_9 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var rcs_el0_trapshadow#1011:u8
        let s_59_0: bool = fn_state.rcs_el0_trapshadow_1011;
        // D s_59_1: write-var gs#141862 <= s_59_0
        fn_state.gs_141862 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#141862:u8
        let s_60_0: bool = fn_state.gs_141862;
        // N s_60_1: branch s_60_0 b63 b61
        if s_60_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #24u : u8
        let s_63_0: u8 = 24;
        // C s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 8u16);
        // C s_63_2: cast zx s_63_1 -> i
        let s_63_2: i128 = (s_63_1.value() as i128);
        // C s_63_3: cast reint s_63_2 -> i64
        let s_63_3: i64 = (s_63_2 as i64);
        // C s_63_4: cast zx s_63_3 -> i
        let s_63_4: i128 = (i128::try_from(s_63_3).unwrap());
        // C s_63_5: const #432u : u32
        let s_63_5: u32 = 432;
        // D s_63_6: read-reg s_63_5:u8
        let s_63_6: u8 = {
            let value = state.read_register::<u8>(s_63_5 as isize);
            tracer.read_register(s_63_5 as isize, value);
            value
        };
        // D s_63_7: call AArch64_SystemAccessTrap(s_63_6, s_63_4)
        let s_63_7: () = AArch64_SystemAccessTrap(state, tracer, s_63_6, s_63_4);
        // N s_63_8: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#141862 <= s_64_0
        fn_state.gs_141862 = s_64_0;
        // N s_64_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var crn:u8
        let s_65_0: u8 = fn_state.crn;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 4u16);
        // C s_65_2: const #11u : u8
        let s_65_2: u8 = 11;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 4u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // N s_65_5: branch s_65_4 b68 b66
        if s_65_4 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var crn:u8
        let s_66_0: u8 = fn_state.crn;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 4u16);
        // C s_66_2: const #15u : u8
        let s_66_2: u8 = 15;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 4u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#141847 <= s_66_4
        fn_state.gs_141847 = s_66_4;
        // N s_66_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#141847:u8
        let s_67_0: bool = fn_state.gs_141847;
        // D s_67_1: write-var gs#141848 <= s_67_0
        fn_state.gs_141848 = s_67_0;
        // N s_67_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#141847 <= s_68_0
        fn_state.gs_141847 = s_68_0;
        // N s_68_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var op0:u8
        let s_69_0: u8 = fn_state.op0;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 2u16);
        // C s_69_2: const #1u : u8
        let s_69_2: u8 = 1;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 2u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // N s_69_5: branch s_69_4 b72 b70
        if s_69_4 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var op0:u8
        let s_70_0: u8 = fn_state.op0;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 2u16);
        // C s_70_2: const #3u : u8
        let s_70_2: u8 = 3;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 2u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#141845 <= s_70_4
        fn_state.gs_141845 = s_70_4;
        // N s_70_6: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#141845:u8
        let s_71_0: bool = fn_state.gs_141845;
        // D s_71_1: write-var gs#141846 <= s_71_0
        fn_state.gs_141846 = s_71_0;
        // N s_71_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#141845 <= s_72_0
        fn_state.gs_141845 = s_72_0;
        // N s_72_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #102552u : u32
        let s_73_0: u32 = 102552;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call _get_HCR_EL2_Type_TIDCP(s_73_1)
        let s_73_2: bool = u_get_HCR_EL2_Type_TIDCP(state, tracer, s_73_1);
        // D s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // C s_73_4: const #1u : u8
        let s_73_4: bool = true;
        // C s_73_5: cast zx s_73_4 -> bv
        let s_73_5: Bits = Bits::new(s_73_4 as u128, 1u16);
        // D s_73_6: cmp-eq s_73_3 s_73_5
        let s_73_6: bool = ((s_73_3) == (s_73_5));
        // D s_73_7: write-var gs#141844 <= s_73_6
        fn_state.gs_141844 = s_73_6;
        // N s_73_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call EL2Enabled(s_74_0)
        let s_74_1: bool = EL2Enabled(state, tracer, s_74_0);
        // D s_74_2: write-var gs#141843 <= s_74_1
        fn_state.gs_141843 = s_74_1;
        // N s_74_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#141842 <= s_75_0
        fn_state.gs_141842 = s_75_0;
        // N s_75_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call IsInHost(s_76_0)
        let s_76_1: bool = IsInHost(state, tracer, s_76_0);
        // S s_76_2: not s_76_1
        let s_76_2: bool = !s_76_1;
        // N s_76_3: branch s_76_2 b93 b77
        if s_76_2 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#141865 <= s_77_0
        fn_state.gs_141865 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#141865:u8
        let s_78_0: bool = fn_state.gs_141865;
        // N s_78_1: branch s_78_0 b87 b79
        if s_78_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call IsInHost(s_80_0)
        let s_80_1: bool = IsInHost(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b86 b81
        if s_80_1 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#141866 <= s_81_0
        fn_state.gs_141866 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#141866:u8
        let s_82_0: bool = fn_state.gs_141866;
        // N s_82_1: branch s_82_0 b85 b83
        if s_82_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_83_0: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_84_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #24u : u8
        let s_85_0: u8 = 24;
        // C s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 8u16);
        // C s_85_2: cast zx s_85_1 -> i
        let s_85_2: i128 = (s_85_1.value() as i128);
        // C s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (i128::try_from(s_85_3).unwrap());
        // C s_85_5: const #432u : u32
        let s_85_5: u32 = 432;
        // D s_85_6: read-reg s_85_5:u8
        let s_85_6: u8 = {
            let value = state.read_register::<u8>(s_85_5 as isize);
            tracer.read_register(s_85_5 as isize, value);
            value
        };
        // D s_85_7: call AArch64_SystemAccessTrap(s_85_6, s_85_4)
        let s_85_7: () = AArch64_SystemAccessTrap(state, tracer, s_85_6, s_85_4);
        // N s_85_8: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #20784u : u32
        let s_86_0: u32 = 20784;
        // D s_86_1: read-reg s_86_0:struct
        let s_86_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // D s_86_2: call _get_SCTLR_EL2_Type_TIDCP(s_86_1)
        let s_86_2: bool = u_get_SCTLR_EL2_Type_TIDCP(state, tracer, s_86_1);
        // D s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // C s_86_4: const #1u : u8
        let s_86_4: bool = true;
        // C s_86_5: cast zx s_86_4 -> bv
        let s_86_5: Bits = Bits::new(s_86_4 as u128, 1u16);
        // D s_86_6: cmp-eq s_86_3 s_86_5
        let s_86_6: bool = ((s_86_3) == (s_86_5));
        // D s_86_7: write-var gs#141866 <= s_86_6
        fn_state.gs_141866 = s_86_6;
        // N s_86_8: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #() : ()
        let s_87_0: () = ();
        // S s_87_1: call EL2Enabled(s_87_0)
        let s_87_1: bool = EL2Enabled(state, tracer, s_87_0);
        // N s_87_2: branch s_87_1 b92 b88
        if s_87_1 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#141869 <= s_88_0
        fn_state.gs_141869 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#141869:u8
        let s_89_0: bool = fn_state.gs_141869;
        // N s_89_1: branch s_89_0 b91 b90
        if s_89_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #24u : u8
        let s_90_0: u8 = 24;
        // C s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 8u16);
        // C s_90_2: cast zx s_90_1 -> i
        let s_90_2: i128 = (s_90_1.value() as i128);
        // C s_90_3: cast reint s_90_2 -> i64
        let s_90_3: i64 = (s_90_2 as i64);
        // C s_90_4: cast zx s_90_3 -> i
        let s_90_4: i128 = (i128::try_from(s_90_3).unwrap());
        // C s_90_5: const #440u : u32
        let s_90_5: u32 = 440;
        // D s_90_6: read-reg s_90_5:u8
        let s_90_6: u8 = {
            let value = state.read_register::<u8>(s_90_5 as isize);
            tracer.read_register(s_90_5 as isize, value);
            value
        };
        // D s_90_7: call AArch64_SystemAccessTrap(s_90_6, s_90_4)
        let s_90_7: () = AArch64_SystemAccessTrap(state, tracer, s_90_6, s_90_4);
        // N s_90_8: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #24u : u8
        let s_91_0: u8 = 24;
        // C s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 8u16);
        // C s_91_2: cast zx s_91_1 -> i
        let s_91_2: i128 = (s_91_1.value() as i128);
        // C s_91_3: cast reint s_91_2 -> i64
        let s_91_3: i64 = (s_91_2 as i64);
        // C s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (i128::try_from(s_91_3).unwrap());
        // C s_91_5: const #432u : u32
        let s_91_5: u32 = 432;
        // D s_91_6: read-reg s_91_5:u8
        let s_91_6: u8 = {
            let value = state.read_register::<u8>(s_91_5 as isize);
            tracer.read_register(s_91_5 as isize, value);
            value
        };
        // D s_91_7: call AArch64_SystemAccessTrap(s_91_6, s_91_4)
        let s_91_7: () = AArch64_SystemAccessTrap(state, tracer, s_91_6, s_91_4);
        // N s_91_8: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #102552u : u32
        let s_92_0: u32 = 102552;
        // D s_92_1: read-reg s_92_0:struct
        let s_92_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call _get_HCR_EL2_Type_TGE(s_92_1)
        let s_92_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_92_1);
        // D s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // C s_92_4: const #1u : u8
        let s_92_4: bool = true;
        // C s_92_5: cast zx s_92_4 -> bv
        let s_92_5: Bits = Bits::new(s_92_4 as u128, 1u16);
        // D s_92_6: cmp-eq s_92_3 s_92_5
        let s_92_6: bool = ((s_92_3) == (s_92_5));
        // D s_92_7: write-var gs#141869 <= s_92_6
        fn_state.gs_141869 = s_92_6;
        // N s_92_8: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #90272u : u32
        let s_93_0: u32 = 90272;
        // D s_93_1: read-reg s_93_0:struct
        let s_93_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call _get_SCTLR_EL1_Type_TIDCP(s_93_1)
        let s_93_2: bool = u_get_SCTLR_EL1_Type_TIDCP(state, tracer, s_93_1);
        // D s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // C s_93_4: const #1u : u8
        let s_93_4: bool = true;
        // C s_93_5: cast zx s_93_4 -> bv
        let s_93_5: Bits = Bits::new(s_93_4 as u128, 1u16);
        // D s_93_6: cmp-eq s_93_3 s_93_5
        let s_93_6: bool = ((s_93_3) == (s_93_5));
        // D s_93_7: write-var gs#141865 <= s_93_6
        fn_state.gs_141865 = s_93_6;
        // N s_93_8: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var crn:u8
        let s_94_0: u8 = fn_state.crn;
        // D s_94_1: write-var b__0 <= s_94_0
        fn_state.b__0 = s_94_0;
        // C s_94_2: const #3s : i
        let s_94_2: i128 = 3;
        // D s_94_3: read-var b__0:u8
        let s_94_3: u8 = fn_state.b__0;
        // D s_94_4: cast zx s_94_3 -> bv
        let s_94_4: Bits = Bits::new(s_94_3 as u128, 4u16);
        // C s_94_5: const #1s : i64
        let s_94_5: i64 = 1;
        // C s_94_6: cast zx s_94_5 -> i
        let s_94_6: i128 = (i128::try_from(s_94_5).unwrap());
        // C s_94_7: const #0s : i
        let s_94_7: i128 = 0;
        // C s_94_8: add s_94_7 s_94_6
        let s_94_8: i128 = (s_94_7 + s_94_6);
        // D s_94_9: bit-extract s_94_4 s_94_2 s_94_8
        let s_94_9: Bits = (Bits::new(
            ((s_94_4) >> (s_94_2)).value(),
            u16::try_from(s_94_8).unwrap(),
        ));
        // D s_94_10: cast reint s_94_9 -> u8
        let s_94_10: bool = ((s_94_9.value()) != 0);
        // D s_94_11: cast zx s_94_10 -> bv
        let s_94_11: Bits = Bits::new(s_94_10 as u128, 1u16);
        // C s_94_12: const #1u : u8
        let s_94_12: bool = true;
        // C s_94_13: cast zx s_94_12 -> bv
        let s_94_13: Bits = Bits::new(s_94_12 as u128, 1u16);
        // D s_94_14: cmp-eq s_94_11 s_94_13
        let s_94_14: bool = ((s_94_11) == (s_94_13));
        // N s_94_15: branch s_94_14 b100 b95
        if s_94_14 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#141838 <= s_95_0
        fn_state.gs_141838 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#141838:u8
        let s_96_0: bool = fn_state.gs_141838;
        // D s_96_1: not s_96_0
        let s_96_1: bool = !s_96_0;
        // N s_96_2: branch s_96_1 b99 b97
        if s_96_1 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #1u : u8
        let s_97_0: bool = true;
        // D s_97_1: write-var gs#141833 <= s_97_0
        fn_state.gs_141833 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#141833:u8
        let s_98_0: bool = fn_state.gs_141833;
        // D s_98_1: write-var gs#141841 <= s_98_0
        fn_state.gs_141841 = s_98_0;
        // N s_98_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#141833 <= s_99_0
        fn_state.gs_141833 = s_99_0;
        // N s_99_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0s : i
        let s_100_0: i128 = 0;
        // D s_100_1: read-var b__0:u8
        let s_100_1: u8 = fn_state.b__0;
        // D s_100_2: cast zx s_100_1 -> bv
        let s_100_2: Bits = Bits::new(s_100_1 as u128, 4u16);
        // C s_100_3: const #1s : i64
        let s_100_3: i64 = 1;
        // C s_100_4: cast zx s_100_3 -> i
        let s_100_4: i128 = (i128::try_from(s_100_3).unwrap());
        // C s_100_5: const #1s : i
        let s_100_5: i128 = 1;
        // C s_100_6: add s_100_5 s_100_4
        let s_100_6: i128 = (s_100_5 + s_100_4);
        // D s_100_7: bit-extract s_100_2 s_100_0 s_100_6
        let s_100_7: Bits = (Bits::new(
            ((s_100_2) >> (s_100_0)).value(),
            u16::try_from(s_100_6).unwrap(),
        ));
        // D s_100_8: cast reint s_100_7 -> u8
        let s_100_8: u8 = (s_100_7.value() as u8);
        // D s_100_9: cast zx s_100_8 -> bv
        let s_100_9: Bits = Bits::new(s_100_8 as u128, 2u16);
        // C s_100_10: const #3u : u8
        let s_100_10: u8 = 3;
        // C s_100_11: cast zx s_100_10 -> bv
        let s_100_11: Bits = Bits::new(s_100_10 as u128, 2u16);
        // D s_100_12: cmp-eq s_100_9 s_100_11
        let s_100_12: bool = ((s_100_9) == (s_100_11));
        // D s_100_13: write-var gs#141838 <= s_100_12
        fn_state.gs_141838 = s_100_12;
        // N s_100_14: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #16975u : u32
        let s_101_0: u32 = 16975;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: cast zx s_101_1 -> bv
        let s_101_2: Bits = Bits::new(s_101_1 as u128, 2u16);
        // C s_101_3: const #448u : u32
        let s_101_3: u32 = 448;
        // D s_101_4: read-reg s_101_3:u8
        let s_101_4: u8 = {
            let value = state.read_register::<u8>(s_101_3 as isize);
            tracer.read_register(s_101_3 as isize, value);
            value
        };
        // D s_101_5: cast zx s_101_4 -> bv
        let s_101_5: Bits = Bits::new(s_101_4 as u128, 2u16);
        // D s_101_6: cmp-eq s_101_2 s_101_5
        let s_101_6: bool = ((s_101_2) == (s_101_5));
        // D s_101_7: write-var gs#141832 <= s_101_6
        fn_state.gs_141832 = s_101_6;
        // N s_101_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #16975u : u32
        let s_102_0: u32 = 16975;
        // D s_102_1: read-reg s_102_0:u8
        let s_102_1: u8 = {
            let value = state.read_register::<u8>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // D s_102_2: cast zx s_102_1 -> bv
        let s_102_2: Bits = Bits::new(s_102_1 as u128, 2u16);
        // C s_102_3: const #448u : u32
        let s_102_3: u32 = 448;
        // D s_102_4: read-reg s_102_3:u8
        let s_102_4: u8 = {
            let value = state.read_register::<u8>(s_102_3 as isize);
            tracer.read_register(s_102_3 as isize, value);
            value
        };
        // D s_102_5: cast zx s_102_4 -> bv
        let s_102_5: Bits = Bits::new(s_102_4 as u128, 2u16);
        // D s_102_6: cmp-eq s_102_2 s_102_5
        let s_102_6: bool = ((s_102_2) == (s_102_5));
        // N s_102_7: branch s_102_6 b118 b103
        if s_102_6 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #16975u : u32
        let s_103_0: u32 = 16975;
        // D s_103_1: read-reg s_103_0:u8
        let s_103_1: u8 = {
            let value = state.read_register::<u8>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // D s_103_2: cast zx s_103_1 -> bv
        let s_103_2: Bits = Bits::new(s_103_1 as u128, 2u16);
        // C s_103_3: const #440u : u32
        let s_103_3: u32 = 440;
        // D s_103_4: read-reg s_103_3:u8
        let s_103_4: u8 = {
            let value = state.read_register::<u8>(s_103_3 as isize);
            tracer.read_register(s_103_3 as isize, value);
            value
        };
        // D s_103_5: cast zx s_103_4 -> bv
        let s_103_5: Bits = Bits::new(s_103_4 as u128, 2u16);
        // D s_103_6: cmp-eq s_103_2 s_103_5
        let s_103_6: bool = ((s_103_2) == (s_103_5));
        // N s_103_7: branch s_103_6 b109 b104
        if s_103_6 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #16975u : u32
        let s_104_0: u32 = 16975;
        // D s_104_1: read-reg s_104_0:u8
        let s_104_1: u8 = {
            let value = state.read_register::<u8>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: cast zx s_104_1 -> bv
        let s_104_2: Bits = Bits::new(s_104_1 as u128, 2u16);
        // C s_104_3: const #432u : u32
        let s_104_3: u32 = 432;
        // D s_104_4: read-reg s_104_3:u8
        let s_104_4: u8 = {
            let value = state.read_register::<u8>(s_104_3 as isize);
            tracer.read_register(s_104_3 as isize, value);
            value
        };
        // D s_104_5: cast zx s_104_4 -> bv
        let s_104_5: Bits = Bits::new(s_104_4 as u128, 2u16);
        // D s_104_6: cmp-eq s_104_2 s_104_5
        let s_104_6: bool = ((s_104_2) == (s_104_5));
        // N s_104_7: branch s_104_6 b108 b105
        if s_104_6 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #16975u : u32
        let s_105_0: u32 = 16975;
        // D s_105_1: read-reg s_105_0:u8
        let s_105_1: u8 = {
            let value = state.read_register::<u8>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // D s_105_2: cast zx s_105_1 -> bv
        let s_105_2: Bits = Bits::new(s_105_1 as u128, 2u16);
        // C s_105_3: const #424u : u32
        let s_105_3: u32 = 424;
        // D s_105_4: read-reg s_105_3:u8
        let s_105_4: u8 = {
            let value = state.read_register::<u8>(s_105_3 as isize);
            tracer.read_register(s_105_3 as isize, value);
            value
        };
        // D s_105_5: cast zx s_105_4 -> bv
        let s_105_5: Bits = Bits::new(s_105_4 as u128, 2u16);
        // D s_105_6: cmp-eq s_105_2 s_105_5
        let s_105_6: bool = ((s_105_2) == (s_105_5));
        // N s_105_7: branch s_105_6 b107 b106
        if s_105_6 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_106_0: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #424u : u32
        let s_107_0: u32 = 424;
        // D s_107_1: read-reg s_107_0:u8
        let s_107_1: u8 = {
            let value = state.read_register::<u8>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: read-var ec_valshadow#1009:i
        let s_107_2: i128 = fn_state.ec_valshadow_1009;
        // D s_107_3: call AArch64_SystemAccessTrap(s_107_1, s_107_2)
        let s_107_3: () = AArch64_SystemAccessTrap(state, tracer, s_107_1, s_107_2);
        // N s_107_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #432u : u32
        let s_108_0: u32 = 432;
        // D s_108_1: read-reg s_108_0:u8
        let s_108_1: u8 = {
            let value = state.read_register::<u8>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // D s_108_2: read-var ec_valshadow#1009:i
        let s_108_2: i128 = fn_state.ec_valshadow_1009;
        // D s_108_3: call AArch64_SystemAccessTrap(s_108_1, s_108_2)
        let s_108_3: () = AArch64_SystemAccessTrap(state, tracer, s_108_1, s_108_2);
        // N s_108_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EL2Enabled(s_109_0)
        let s_109_1: bool = EL2Enabled(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b117 b110
        if s_109_1 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#141871 <= s_110_0
        fn_state.gs_141871 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#141871:u8
        let s_111_0: bool = fn_state.gs_141871;
        // N s_111_1: branch s_111_0 b116 b112
        if s_111_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#141872 <= s_112_0
        fn_state.gs_141872 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#141872:u8
        let s_113_0: bool = fn_state.gs_141872;
        // N s_113_1: branch s_113_0 b115 b114
        if s_113_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #440u : u32
        let s_114_0: u32 = 440;
        // D s_114_1: read-reg s_114_0:u8
        let s_114_1: u8 = {
            let value = state.read_register::<u8>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // D s_114_2: read-var ec_valshadow#1009:i
        let s_114_2: i128 = fn_state.ec_valshadow_1009;
        // D s_114_3: call AArch64_SystemAccessTrap(s_114_1, s_114_2)
        let s_114_3: () = AArch64_SystemAccessTrap(state, tracer, s_114_1, s_114_2);
        // N s_114_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #432u : u32
        let s_115_0: u32 = 432;
        // D s_115_1: read-reg s_115_0:u8
        let s_115_1: u8 = {
            let value = state.read_register::<u8>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // D s_115_2: read-var ec_valshadow#1009:i
        let s_115_2: i128 = fn_state.ec_valshadow_1009;
        // D s_115_3: call AArch64_SystemAccessTrap(s_115_1, s_115_2)
        let s_115_3: () = AArch64_SystemAccessTrap(state, tracer, s_115_1, s_115_2);
        // N s_115_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #102552u : u32
        let s_116_0: u32 = 102552;
        // D s_116_1: read-reg s_116_0:struct
        let s_116_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // D s_116_2: call _get_HCR_EL2_Type_TID3(s_116_1)
        let s_116_2: bool = u_get_HCR_EL2_Type_TID3(state, tracer, s_116_1);
        // D s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // C s_116_4: const #1u : u8
        let s_116_4: bool = true;
        // C s_116_5: cast zx s_116_4 -> bv
        let s_116_5: Bits = Bits::new(s_116_4 as u128, 1u16);
        // D s_116_6: cmp-eq s_116_3 s_116_5
        let s_116_6: bool = ((s_116_3) == (s_116_5));
        // D s_116_7: write-var gs#141872 <= s_116_6
        fn_state.gs_141872 = s_116_6;
        // N s_116_8: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #432u : u32
        let s_117_0: u32 = 432;
        // D s_117_1: read-reg s_117_0:u8
        let s_117_1: u8 = {
            let value = state.read_register::<u8>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // D s_117_2: call ELUsingAArch32(s_117_1)
        let s_117_2: bool = ELUsingAArch32(state, tracer, s_117_1);
        // D s_117_3: not s_117_2
        let s_117_3: bool = !s_117_2;
        // D s_117_4: write-var gs#141871 <= s_117_3
        fn_state.gs_141871 = s_117_3;
        // N s_117_5: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EL2Enabled(s_118_0)
        let s_118_1: bool = EL2Enabled(state, tracer, s_118_0);
        // N s_118_2: branch s_118_1 b126 b119
        if s_118_1 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#141873 <= s_119_0
        fn_state.gs_141873 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#141873:u8
        let s_120_0: bool = fn_state.gs_141873;
        // N s_120_1: branch s_120_0 b125 b121
        if s_120_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#141874 <= s_121_0
        fn_state.gs_141874 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#141874:u8
        let s_122_0: bool = fn_state.gs_141874;
        // N s_122_1: branch s_122_0 b124 b123
        if s_122_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #440u : u32
        let s_123_0: u32 = 440;
        // D s_123_1: read-reg s_123_0:u8
        let s_123_1: u8 = {
            let value = state.read_register::<u8>(s_123_0 as isize);
            tracer.read_register(s_123_0 as isize, value);
            value
        };
        // D s_123_2: read-var ec_valshadow#1009:i
        let s_123_2: i128 = fn_state.ec_valshadow_1009;
        // D s_123_3: call AArch64_SystemAccessTrap(s_123_1, s_123_2)
        let s_123_3: () = AArch64_SystemAccessTrap(state, tracer, s_123_1, s_123_2);
        // N s_123_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #432u : u32
        let s_124_0: u32 = 432;
        // D s_124_1: read-reg s_124_0:u8
        let s_124_1: u8 = {
            let value = state.read_register::<u8>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // D s_124_2: read-var ec_valshadow#1009:i
        let s_124_2: i128 = fn_state.ec_valshadow_1009;
        // D s_124_3: call AArch64_SystemAccessTrap(s_124_1, s_124_2)
        let s_124_3: () = AArch64_SystemAccessTrap(state, tracer, s_124_1, s_124_2);
        // N s_124_4: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #102552u : u32
        let s_125_0: u32 = 102552;
        // D s_125_1: read-reg s_125_0:struct
        let s_125_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call _get_HCR_EL2_Type_TGE(s_125_1)
        let s_125_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_125_1);
        // D s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // C s_125_4: const #1u : u8
        let s_125_4: bool = true;
        // C s_125_5: cast zx s_125_4 -> bv
        let s_125_5: Bits = Bits::new(s_125_4 as u128, 1u16);
        // D s_125_6: cmp-eq s_125_3 s_125_5
        let s_125_6: bool = ((s_125_3) == (s_125_5));
        // D s_125_7: write-var gs#141874 <= s_125_6
        fn_state.gs_141874 = s_125_6;
        // N s_125_8: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #432u : u32
        let s_126_0: u32 = 432;
        // D s_126_1: read-reg s_126_0:u8
        let s_126_1: u8 = {
            let value = state.read_register::<u8>(s_126_0 as isize);
            tracer.read_register(s_126_0 as isize, value);
            value
        };
        // D s_126_2: call ELUsingAArch32(s_126_1)
        let s_126_2: bool = ELUsingAArch32(state, tracer, s_126_1);
        // D s_126_3: not s_126_2
        let s_126_3: bool = !s_126_2;
        // D s_126_4: write-var gs#141873 <= s_126_3
        fn_state.gs_141873 = s_126_3;
        // N s_126_5: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var read:u8
        let s_127_0: bool = fn_state.read;
        // D s_127_1: write-var gs#141831 <= s_127_0
        fn_state.gs_141831 = s_127_0;
        // N s_127_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #24u : u8
        let s_128_0: u8 = 24;
        // C s_128_1: cast zx s_128_0 -> bv
        let s_128_1: Bits = Bits::new(s_128_0 as u128, 8u16);
        // C s_128_2: cast zx s_128_1 -> i
        let s_128_2: i128 = (s_128_1.value() as i128);
        // D s_128_3: write-var ec_val <= s_128_2
        fn_state.ec_val = s_128_2;
        // N s_128_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var crm:u8
        let s_129_0: u8 = fn_state.crm;
        // C s_129_1: const #3s : i
        let s_129_1: i128 = 3;
        // D s_129_2: cast zx s_129_0 -> bv
        let s_129_2: Bits = Bits::new(s_129_0 as u128, 4u16);
        // C s_129_3: const #1s : i64
        let s_129_3: i64 = 1;
        // C s_129_4: cast zx s_129_3 -> i
        let s_129_4: i128 = (i128::try_from(s_129_3).unwrap());
        // C s_129_5: const #0s : i
        let s_129_5: i128 = 0;
        // C s_129_6: add s_129_5 s_129_4
        let s_129_6: i128 = (s_129_5 + s_129_4);
        // D s_129_7: bit-extract s_129_2 s_129_1 s_129_6
        let s_129_7: Bits = (Bits::new(
            ((s_129_2) >> (s_129_1)).value(),
            u16::try_from(s_129_6).unwrap(),
        ));
        // D s_129_8: cast reint s_129_7 -> u8
        let s_129_8: bool = ((s_129_7.value()) != 0);
        // D s_129_9: cast zx s_129_8 -> bv
        let s_129_9: Bits = Bits::new(s_129_8 as u128, 1u16);
        // C s_129_10: const #0u : u8
        let s_129_10: bool = false;
        // C s_129_11: cast zx s_129_10 -> bv
        let s_129_11: Bits = Bits::new(s_129_10 as u128, 1u16);
        // D s_129_12: cmp-eq s_129_9 s_129_11
        let s_129_12: bool = ((s_129_9) == (s_129_11));
        // D s_129_13: not s_129_12
        let s_129_13: bool = !s_129_12;
        // N s_129_14: branch s_129_13 b132 b130
        if s_129_13 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #1u : u8
        let s_130_0: bool = true;
        // D s_130_1: write-var gs#141823 <= s_130_0
        fn_state.gs_141823 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#141823:u8
        let s_131_0: bool = fn_state.gs_141823;
        // D s_131_1: write-var gs#141828 <= s_131_0
        fn_state.gs_141828 = s_131_0;
        // N s_131_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#141823 <= s_132_0
        fn_state.gs_141823 = s_132_0;
        // N s_132_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var crn:u8
        let s_133_0: u8 = fn_state.crn;
        // D s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 4u16);
        // C s_133_2: const #0u : u8
        let s_133_2: u8 = 0;
        // C s_133_3: cast zx s_133_2 -> bv
        let s_133_3: Bits = Bits::new(s_133_2 as u128, 4u16);
        // D s_133_4: cmp-eq s_133_1 s_133_3
        let s_133_4: bool = ((s_133_1) == (s_133_3));
        // D s_133_5: write-var gs#141822 <= s_133_4
        fn_state.gs_141822 = s_133_4;
        // N s_133_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var op1:u8
        let s_134_0: u8 = fn_state.op1;
        // C s_134_1: const #1s : i
        let s_134_1: i128 = 1;
        // D s_134_2: cast zx s_134_0 -> bv
        let s_134_2: Bits = Bits::new(s_134_0 as u128, 3u16);
        // C s_134_3: const #1s : i64
        let s_134_3: i64 = 1;
        // C s_134_4: cast zx s_134_3 -> i
        let s_134_4: i128 = (i128::try_from(s_134_3).unwrap());
        // C s_134_5: const #1s : i
        let s_134_5: i128 = 1;
        // C s_134_6: add s_134_5 s_134_4
        let s_134_6: i128 = (s_134_5 + s_134_4);
        // D s_134_7: bit-extract s_134_2 s_134_1 s_134_6
        let s_134_7: Bits = (Bits::new(
            ((s_134_2) >> (s_134_1)).value(),
            u16::try_from(s_134_6).unwrap(),
        ));
        // D s_134_8: cast reint s_134_7 -> u8
        let s_134_8: u8 = (s_134_7.value() as u8);
        // D s_134_9: cast zx s_134_8 -> bv
        let s_134_9: Bits = Bits::new(s_134_8 as u128, 2u16);
        // C s_134_10: const #0u : u8
        let s_134_10: u8 = 0;
        // C s_134_11: cast zx s_134_10 -> bv
        let s_134_11: Bits = Bits::new(s_134_10 as u128, 2u16);
        // D s_134_12: cmp-eq s_134_9 s_134_11
        let s_134_12: bool = ((s_134_9) == (s_134_11));
        // D s_134_13: not s_134_12
        let s_134_13: bool = !s_134_12;
        // N s_134_14: branch s_134_13 b137 b135
        if s_134_13 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #1u : u8
        let s_135_0: bool = true;
        // D s_135_1: write-var gs#141815 <= s_135_0
        fn_state.gs_141815 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#141815:u8
        let s_136_0: bool = fn_state.gs_141815;
        // D s_136_1: write-var gs#141821 <= s_136_0
        fn_state.gs_141821 = s_136_0;
        // N s_136_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var op1:u8
        let s_137_0: u8 = fn_state.op1;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 3u16);
        // C s_137_2: const #3u : u8
        let s_137_2: u8 = 3;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 3u16);
        // D s_137_4: cmp-eq s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) == (s_137_3));
        // D s_137_5: not s_137_4
        let s_137_5: bool = !s_137_4;
        // N s_137_6: branch s_137_5 b139 b138
        if s_137_5 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #1u : u8
        let s_138_0: bool = true;
        // D s_138_1: write-var gs#141815 <= s_138_0
        fn_state.gs_141815 = s_138_0;
        // N s_138_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #0u : u8
        let s_139_0: bool = false;
        // D s_139_1: write-var gs#141815 <= s_139_0
        fn_state.gs_141815 = s_139_0;
        // N s_139_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var op0:u8
        let s_140_0: u8 = fn_state.op0;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 2u16);
        // C s_140_2: const #3u : u8
        let s_140_2: u8 = 3;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 2u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // N s_140_5: branch s_140_4 b248 b141
        if s_140_4 {
            return block_248(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #0u : u8
        let s_141_0: bool = false;
        // D s_141_1: write-var gs#141768 <= s_141_0
        fn_state.gs_141768 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#141768:u8
        let s_142_0: bool = fn_state.gs_141768;
        // N s_142_1: branch s_142_0 b247 b143
        if s_142_0 {
            return block_247(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#141769 <= s_143_0
        fn_state.gs_141769 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#141769:u8
        let s_144_0: bool = fn_state.gs_141769;
        // N s_144_1: branch s_144_0 b147 b145
        if s_144_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_145_0: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_146_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var crm:u8
        let s_147_0: u8 = fn_state.crm;
        // D s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 4u16);
        // C s_147_2: const #2u : u8
        let s_147_2: u8 = 2;
        // C s_147_3: cast zx s_147_2 -> bv
        let s_147_3: Bits = Bits::new(s_147_2 as u128, 4u16);
        // D s_147_4: cmp-eq s_147_1 s_147_3
        let s_147_4: bool = ((s_147_1) == (s_147_3));
        // N s_147_5: branch s_147_4 b246 b148
        if s_147_4 {
            return block_246(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#141771 <= s_148_0
        fn_state.gs_141771 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#141771:u8
        let s_149_0: bool = fn_state.gs_141771;
        // N s_149_1: branch s_149_0 b243 b150
        if s_149_0 {
            return block_243(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var crm:u8
        let s_150_0: u8 = fn_state.crm;
        // D s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 4u16);
        // C s_150_2: const #3u : u8
        let s_150_2: u8 = 3;
        // C s_150_3: cast zx s_150_2 -> bv
        let s_150_3: Bits = Bits::new(s_150_2 as u128, 4u16);
        // D s_150_4: cmp-eq s_150_1 s_150_3
        let s_150_4: bool = ((s_150_1) == (s_150_3));
        // N s_150_5: branch s_150_4 b233 b151
        if s_150_4 {
            return block_233(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#141775 <= s_151_0
        fn_state.gs_141775 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#141775:u8
        let s_152_0: bool = fn_state.gs_141775;
        // N s_152_1: branch s_152_0 b230 b153
        if s_152_0 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var crm:u8
        let s_153_0: u8 = fn_state.crm;
        // D s_153_1: cast zx s_153_0 -> bv
        let s_153_1: Bits = Bits::new(s_153_0 as u128, 4u16);
        // C s_153_2: const #4u : u8
        let s_153_2: u8 = 4;
        // C s_153_3: cast zx s_153_2 -> bv
        let s_153_3: Bits = Bits::new(s_153_2 as u128, 4u16);
        // D s_153_4: cmp-eq s_153_1 s_153_3
        let s_153_4: bool = ((s_153_1) == (s_153_3));
        // N s_153_5: branch s_153_4 b217 b154
        if s_153_4 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#141780 <= s_154_0
        fn_state.gs_141780 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#141780:u8
        let s_155_0: bool = fn_state.gs_141780;
        // N s_155_1: branch s_155_0 b214 b156
        if s_155_0 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var crm:u8
        let s_156_0: u8 = fn_state.crm;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 4u16);
        // C s_156_2: const #5u : u8
        let s_156_2: u8 = 5;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 4u16);
        // D s_156_4: cmp-eq s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) == (s_156_3));
        // N s_156_5: branch s_156_4 b204 b157
        if s_156_4 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #0u : u8
        let s_157_0: bool = false;
        // D s_157_1: write-var gs#141784 <= s_157_0
        fn_state.gs_141784 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#141784:u8
        let s_158_0: bool = fn_state.gs_141784;
        // N s_158_1: branch s_158_0 b201 b159
        if s_158_0 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var crm:u8
        let s_159_0: u8 = fn_state.crm;
        // D s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 4u16);
        // C s_159_2: const #6u : u8
        let s_159_2: u8 = 6;
        // C s_159_3: cast zx s_159_2 -> bv
        let s_159_3: Bits = Bits::new(s_159_2 as u128, 4u16);
        // D s_159_4: cmp-eq s_159_1 s_159_3
        let s_159_4: bool = ((s_159_1) == (s_159_3));
        // N s_159_5: branch s_159_4 b185 b160
        if s_159_4 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #0u : u8
        let s_160_0: bool = false;
        // D s_160_1: write-var gs#141790 <= s_160_0
        fn_state.gs_141790 = s_160_0;
        // N s_160_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var gs#141790:u8
        let s_161_0: bool = fn_state.gs_141790;
        // N s_161_1: branch s_161_0 b182 b162
        if s_161_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var crm:u8
        let s_162_0: u8 = fn_state.crm;
        // D s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 4u16);
        // C s_162_2: const #7u : u8
        let s_162_2: u8 = 7;
        // C s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 4u16);
        // D s_162_4: cmp-eq s_162_1 s_162_3
        let s_162_4: bool = ((s_162_1) == (s_162_3));
        // N s_162_5: branch s_162_4 b169 b163
        if s_162_4 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #0u : u8
        let s_163_0: bool = false;
        // D s_163_1: write-var gs#141795 <= s_163_0
        fn_state.gs_141795 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#141795:u8
        let s_164_0: bool = fn_state.gs_141795;
        // N s_164_1: branch s_164_0 b166 b165
        if s_164_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_165_0: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var tid3_trap:u8
        let s_166_0: bool = fn_state.tid3_trap;
        // N s_166_1: branch s_166_0 b168 b167
        if s_166_0 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #64s : i64
        let s_167_0: i64 = 64;
        // C s_167_1: const #64s : i
        let s_167_1: i128 = 64;
        // S s_167_2: call Zeros(s_167_1)
        let s_167_2: Bits = Zeros(state, tracer, s_167_1);
        // S s_167_3: cast reint s_167_2 -> u64
        let s_167_3: u64 = (s_167_2.value() as u64);
        // S s_167_4: cast zx s_167_3 -> bv
        let s_167_4: Bits = Bits::new(s_167_3 as u128, 64u16);
        // D s_167_5: read-var t:i
        let s_167_5: i128 = fn_state.t;
        // D s_167_6: call X_set(s_167_5, s_167_0, s_167_4)
        let s_167_6: () = X_set(state, tracer, s_167_5, s_167_0, s_167_4);
        // N s_167_7: return
        return;
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #24u : u8
        let s_168_0: u8 = 24;
        // C s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 8u16);
        // C s_168_2: cast zx s_168_1 -> i
        let s_168_2: i128 = (s_168_1.value() as i128);
        // C s_168_3: cast reint s_168_2 -> i64
        let s_168_3: i64 = (s_168_2 as i64);
        // C s_168_4: cast zx s_168_3 -> i
        let s_168_4: i128 = (i128::try_from(s_168_3).unwrap());
        // C s_168_5: const #432u : u32
        let s_168_5: u32 = 432;
        // D s_168_6: read-reg s_168_5:u8
        let s_168_6: u8 = {
            let value = state.read_register::<u8>(s_168_5 as isize);
            tracer.read_register(s_168_5 as isize, value);
            value
        };
        // D s_168_7: call AArch64_SystemAccessTrap(s_168_6, s_168_4)
        let s_168_7: () = AArch64_SystemAccessTrap(state, tracer, s_168_6, s_168_4);
        // N s_168_8: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var op2:u8
        let s_169_0: u8 = fn_state.op2;
        // D s_169_1: cast zx s_169_0 -> bv
        let s_169_1: Bits = Bits::new(s_169_0 as u128, 3u16);
        // C s_169_2: const #3u : u8
        let s_169_2: u8 = 3;
        // C s_169_3: cast zx s_169_2 -> bv
        let s_169_3: Bits = Bits::new(s_169_2 as u128, 3u16);
        // D s_169_4: cmp-eq s_169_1 s_169_3
        let s_169_4: bool = ((s_169_1) == (s_169_3));
        // N s_169_5: branch s_169_4 b181 b170
        if s_169_4 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var op2:u8
        let s_170_0: u8 = fn_state.op2;
        // D s_170_1: cast zx s_170_0 -> bv
        let s_170_1: Bits = Bits::new(s_170_0 as u128, 3u16);
        // C s_170_2: const #4u : u8
        let s_170_2: u8 = 4;
        // C s_170_3: cast zx s_170_2 -> bv
        let s_170_3: Bits = Bits::new(s_170_2 as u128, 3u16);
        // D s_170_4: cmp-eq s_170_1 s_170_3
        let s_170_4: bool = ((s_170_1) == (s_170_3));
        // N s_170_5: branch s_170_4 b180 b171
        if s_170_4 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var op2:u8
        let s_171_0: u8 = fn_state.op2;
        // D s_171_1: cast zx s_171_0 -> bv
        let s_171_1: Bits = Bits::new(s_171_0 as u128, 3u16);
        // C s_171_2: const #5u : u8
        let s_171_2: u8 = 5;
        // C s_171_3: cast zx s_171_2 -> bv
        let s_171_3: Bits = Bits::new(s_171_2 as u128, 3u16);
        // D s_171_4: cmp-eq s_171_1 s_171_3
        let s_171_4: bool = ((s_171_1) == (s_171_3));
        // N s_171_5: branch s_171_4 b179 b172
        if s_171_4 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var op2:u8
        let s_172_0: u8 = fn_state.op2;
        // D s_172_1: cast zx s_172_0 -> bv
        let s_172_1: Bits = Bits::new(s_172_0 as u128, 3u16);
        // C s_172_2: const #6u : u8
        let s_172_2: u8 = 6;
        // C s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 3u16);
        // D s_172_4: cmp-eq s_172_1 s_172_3
        let s_172_4: bool = ((s_172_1) == (s_172_3));
        // N s_172_5: branch s_172_4 b178 b173
        if s_172_4 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var op2:u8
        let s_173_0: u8 = fn_state.op2;
        // D s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 3u16);
        // C s_173_2: const #7u : u8
        let s_173_2: u8 = 7;
        // C s_173_3: cast zx s_173_2 -> bv
        let s_173_3: Bits = Bits::new(s_173_2 as u128, 3u16);
        // D s_173_4: cmp-eq s_173_1 s_173_3
        let s_173_4: bool = ((s_173_1) == (s_173_3));
        // D s_173_5: write-var gs#141791 <= s_173_4
        fn_state.gs_141791 = s_173_4;
        // N s_173_6: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#141791:u8
        let s_174_0: bool = fn_state.gs_141791;
        // D s_174_1: write-var gs#141792 <= s_174_0
        fn_state.gs_141792 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var gs#141792:u8
        let s_175_0: bool = fn_state.gs_141792;
        // D s_175_1: write-var gs#141793 <= s_175_0
        fn_state.gs_141793 = s_175_0;
        // N s_175_2: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var gs#141793:u8
        let s_176_0: bool = fn_state.gs_141793;
        // D s_176_1: write-var gs#141794 <= s_176_0
        fn_state.gs_141794 = s_176_0;
        // N s_176_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var gs#141794:u8
        let s_177_0: bool = fn_state.gs_141794;
        // D s_177_1: write-var gs#141795 <= s_177_0
        fn_state.gs_141795 = s_177_0;
        // N s_177_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #1u : u8
        let s_178_0: bool = true;
        // D s_178_1: write-var gs#141791 <= s_178_0
        fn_state.gs_141791 = s_178_0;
        // N s_178_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #1u : u8
        let s_179_0: bool = true;
        // D s_179_1: write-var gs#141792 <= s_179_0
        fn_state.gs_141792 = s_179_0;
        // N s_179_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #1u : u8
        let s_180_0: bool = true;
        // D s_180_1: write-var gs#141793 <= s_180_0
        fn_state.gs_141793 = s_180_0;
        // N s_180_2: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #1u : u8
        let s_181_0: bool = true;
        // D s_181_1: write-var gs#141794 <= s_181_0
        fn_state.gs_141794 = s_181_0;
        // N s_181_2: jump b177
        return block_177(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var tid3_trap:u8
        let s_182_0: bool = fn_state.tid3_trap;
        // N s_182_1: branch s_182_0 b184 b183
        if s_182_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #64s : i64
        let s_183_0: i64 = 64;
        // C s_183_1: const #64s : i
        let s_183_1: i128 = 64;
        // S s_183_2: call Zeros(s_183_1)
        let s_183_2: Bits = Zeros(state, tracer, s_183_1);
        // S s_183_3: cast reint s_183_2 -> u64
        let s_183_3: u64 = (s_183_2.value() as u64);
        // S s_183_4: cast zx s_183_3 -> bv
        let s_183_4: Bits = Bits::new(s_183_3 as u128, 64u16);
        // D s_183_5: read-var t:i
        let s_183_5: i128 = fn_state.t;
        // D s_183_6: call X_set(s_183_5, s_183_0, s_183_4)
        let s_183_6: () = X_set(state, tracer, s_183_5, s_183_0, s_183_4);
        // N s_183_7: return
        return;
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #24u : u8
        let s_184_0: u8 = 24;
        // C s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 8u16);
        // C s_184_2: cast zx s_184_1 -> i
        let s_184_2: i128 = (s_184_1.value() as i128);
        // C s_184_3: cast reint s_184_2 -> i64
        let s_184_3: i64 = (s_184_2 as i64);
        // C s_184_4: cast zx s_184_3 -> i
        let s_184_4: i128 = (i128::try_from(s_184_3).unwrap());
        // C s_184_5: const #432u : u32
        let s_184_5: u32 = 432;
        // D s_184_6: read-reg s_184_5:u8
        let s_184_6: u8 = {
            let value = state.read_register::<u8>(s_184_5 as isize);
            tracer.read_register(s_184_5 as isize, value);
            value
        };
        // D s_184_7: call AArch64_SystemAccessTrap(s_184_6, s_184_4)
        let s_184_7: () = AArch64_SystemAccessTrap(state, tracer, s_184_6, s_184_4);
        // N s_184_8: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_185_0: read-var op2:u8
        let s_185_0: u8 = fn_state.op2;
        // D s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 3u16);
        // C s_185_2: const #2u : u8
        let s_185_2: u8 = 2;
        // C s_185_3: cast zx s_185_2 -> bv
        let s_185_3: Bits = Bits::new(s_185_2 as u128, 3u16);
        // D s_185_4: cmp-eq s_185_1 s_185_3
        let s_185_4: bool = ((s_185_1) == (s_185_3));
        // N s_185_5: branch s_185_4 b200 b186
        if s_185_4 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_186(state, tracer, fn_state);
        };
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var op2:u8
        let s_186_0: u8 = fn_state.op2;
        // D s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 3u16);
        // C s_186_2: const #3u : u8
        let s_186_2: u8 = 3;
        // C s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 3u16);
        // D s_186_4: cmp-eq s_186_1 s_186_3
        let s_186_4: bool = ((s_186_1) == (s_186_3));
        // N s_186_5: branch s_186_4 b199 b187
        if s_186_4 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_187_0: read-var op2:u8
        let s_187_0: u8 = fn_state.op2;
        // D s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 3u16);
        // C s_187_2: const #4u : u8
        let s_187_2: u8 = 4;
        // C s_187_3: cast zx s_187_2 -> bv
        let s_187_3: Bits = Bits::new(s_187_2 as u128, 3u16);
        // D s_187_4: cmp-eq s_187_1 s_187_3
        let s_187_4: bool = ((s_187_1) == (s_187_3));
        // N s_187_5: branch s_187_4 b198 b188
        if s_187_4 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_188(state, tracer, fn_state);
        };
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var op2:u8
        let s_188_0: u8 = fn_state.op2;
        // D s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 3u16);
        // C s_188_2: const #5u : u8
        let s_188_2: u8 = 5;
        // C s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 3u16);
        // D s_188_4: cmp-eq s_188_1 s_188_3
        let s_188_4: bool = ((s_188_1) == (s_188_3));
        // N s_188_5: branch s_188_4 b197 b189
        if s_188_4 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var op2:u8
        let s_189_0: u8 = fn_state.op2;
        // D s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 3u16);
        // C s_189_2: const #6u : u8
        let s_189_2: u8 = 6;
        // C s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 3u16);
        // D s_189_4: cmp-eq s_189_1 s_189_3
        let s_189_4: bool = ((s_189_1) == (s_189_3));
        // N s_189_5: branch s_189_4 b196 b190
        if s_189_4 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var op2:u8
        let s_190_0: u8 = fn_state.op2;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 3u16);
        // C s_190_2: const #7u : u8
        let s_190_2: u8 = 7;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 3u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#141785 <= s_190_4
        fn_state.gs_141785 = s_190_4;
        // N s_190_6: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var gs#141785:u8
        let s_191_0: bool = fn_state.gs_141785;
        // D s_191_1: write-var gs#141786 <= s_191_0
        fn_state.gs_141786 = s_191_0;
        // N s_191_2: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var gs#141786:u8
        let s_192_0: bool = fn_state.gs_141786;
        // D s_192_1: write-var gs#141787 <= s_192_0
        fn_state.gs_141787 = s_192_0;
        // N s_192_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var gs#141787:u8
        let s_193_0: bool = fn_state.gs_141787;
        // D s_193_1: write-var gs#141788 <= s_193_0
        fn_state.gs_141788 = s_193_0;
        // N s_193_2: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var gs#141788:u8
        let s_194_0: bool = fn_state.gs_141788;
        // D s_194_1: write-var gs#141789 <= s_194_0
        fn_state.gs_141789 = s_194_0;
        // N s_194_2: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_195_0: read-var gs#141789:u8
        let s_195_0: bool = fn_state.gs_141789;
        // D s_195_1: write-var gs#141790 <= s_195_0
        fn_state.gs_141790 = s_195_0;
        // N s_195_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #1u : u8
        let s_196_0: bool = true;
        // D s_196_1: write-var gs#141785 <= s_196_0
        fn_state.gs_141785 = s_196_0;
        // N s_196_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #1u : u8
        let s_197_0: bool = true;
        // D s_197_1: write-var gs#141786 <= s_197_0
        fn_state.gs_141786 = s_197_0;
        // N s_197_2: jump b192
        return block_192(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #1u : u8
        let s_198_0: bool = true;
        // D s_198_1: write-var gs#141787 <= s_198_0
        fn_state.gs_141787 = s_198_0;
        // N s_198_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #1u : u8
        let s_199_0: bool = true;
        // D s_199_1: write-var gs#141788 <= s_199_0
        fn_state.gs_141788 = s_199_0;
        // N s_199_2: jump b194
        return block_194(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #1u : u8
        let s_200_0: bool = true;
        // D s_200_1: write-var gs#141789 <= s_200_0
        fn_state.gs_141789 = s_200_0;
        // N s_200_2: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_201_0: read-var tid3_trap:u8
        let s_201_0: bool = fn_state.tid3_trap;
        // N s_201_1: branch s_201_0 b203 b202
        if s_201_0 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_202(state, tracer, fn_state);
        };
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #64s : i64
        let s_202_0: i64 = 64;
        // C s_202_1: const #64s : i
        let s_202_1: i128 = 64;
        // S s_202_2: call Zeros(s_202_1)
        let s_202_2: Bits = Zeros(state, tracer, s_202_1);
        // S s_202_3: cast reint s_202_2 -> u64
        let s_202_3: u64 = (s_202_2.value() as u64);
        // S s_202_4: cast zx s_202_3 -> bv
        let s_202_4: Bits = Bits::new(s_202_3 as u128, 64u16);
        // D s_202_5: read-var t:i
        let s_202_5: i128 = fn_state.t;
        // D s_202_6: call X_set(s_202_5, s_202_0, s_202_4)
        let s_202_6: () = X_set(state, tracer, s_202_5, s_202_0, s_202_4);
        // N s_202_7: return
        return;
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #24u : u8
        let s_203_0: u8 = 24;
        // C s_203_1: cast zx s_203_0 -> bv
        let s_203_1: Bits = Bits::new(s_203_0 as u128, 8u16);
        // C s_203_2: cast zx s_203_1 -> i
        let s_203_2: i128 = (s_203_1.value() as i128);
        // C s_203_3: cast reint s_203_2 -> i64
        let s_203_3: i64 = (s_203_2 as i64);
        // C s_203_4: cast zx s_203_3 -> i
        let s_203_4: i128 = (i128::try_from(s_203_3).unwrap());
        // C s_203_5: const #432u : u32
        let s_203_5: u32 = 432;
        // D s_203_6: read-reg s_203_5:u8
        let s_203_6: u8 = {
            let value = state.read_register::<u8>(s_203_5 as isize);
            tracer.read_register(s_203_5 as isize, value);
            value
        };
        // D s_203_7: call AArch64_SystemAccessTrap(s_203_6, s_203_4)
        let s_203_7: () = AArch64_SystemAccessTrap(state, tracer, s_203_6, s_203_4);
        // N s_203_8: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_204_0: read-var op2:u8
        let s_204_0: u8 = fn_state.op2;
        // D s_204_1: cast zx s_204_0 -> bv
        let s_204_1: Bits = Bits::new(s_204_0 as u128, 3u16);
        // C s_204_2: const #2u : u8
        let s_204_2: u8 = 2;
        // C s_204_3: cast zx s_204_2 -> bv
        let s_204_3: Bits = Bits::new(s_204_2 as u128, 3u16);
        // D s_204_4: cmp-eq s_204_1 s_204_3
        let s_204_4: bool = ((s_204_1) == (s_204_3));
        // N s_204_5: branch s_204_4 b213 b205
        if s_204_4 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var op2:u8
        let s_205_0: u8 = fn_state.op2;
        // D s_205_1: cast zx s_205_0 -> bv
        let s_205_1: Bits = Bits::new(s_205_0 as u128, 3u16);
        // C s_205_2: const #3u : u8
        let s_205_2: u8 = 3;
        // C s_205_3: cast zx s_205_2 -> bv
        let s_205_3: Bits = Bits::new(s_205_2 as u128, 3u16);
        // D s_205_4: cmp-eq s_205_1 s_205_3
        let s_205_4: bool = ((s_205_1) == (s_205_3));
        // N s_205_5: branch s_205_4 b212 b206
        if s_205_4 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_206(state, tracer, fn_state);
        };
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var op2:u8
        let s_206_0: u8 = fn_state.op2;
        // D s_206_1: cast zx s_206_0 -> bv
        let s_206_1: Bits = Bits::new(s_206_0 as u128, 3u16);
        // C s_206_2: const #6u : u8
        let s_206_2: u8 = 6;
        // C s_206_3: cast zx s_206_2 -> bv
        let s_206_3: Bits = Bits::new(s_206_2 as u128, 3u16);
        // D s_206_4: cmp-eq s_206_1 s_206_3
        let s_206_4: bool = ((s_206_1) == (s_206_3));
        // N s_206_5: branch s_206_4 b211 b207
        if s_206_4 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var op2:u8
        let s_207_0: u8 = fn_state.op2;
        // D s_207_1: cast zx s_207_0 -> bv
        let s_207_1: Bits = Bits::new(s_207_0 as u128, 3u16);
        // C s_207_2: const #7u : u8
        let s_207_2: u8 = 7;
        // C s_207_3: cast zx s_207_2 -> bv
        let s_207_3: Bits = Bits::new(s_207_2 as u128, 3u16);
        // D s_207_4: cmp-eq s_207_1 s_207_3
        let s_207_4: bool = ((s_207_1) == (s_207_3));
        // D s_207_5: write-var gs#141781 <= s_207_4
        fn_state.gs_141781 = s_207_4;
        // N s_207_6: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var gs#141781:u8
        let s_208_0: bool = fn_state.gs_141781;
        // D s_208_1: write-var gs#141782 <= s_208_0
        fn_state.gs_141782 = s_208_0;
        // N s_208_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var gs#141782:u8
        let s_209_0: bool = fn_state.gs_141782;
        // D s_209_1: write-var gs#141783 <= s_209_0
        fn_state.gs_141783 = s_209_0;
        // N s_209_2: jump b210
        return block_210(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var gs#141783:u8
        let s_210_0: bool = fn_state.gs_141783;
        // D s_210_1: write-var gs#141784 <= s_210_0
        fn_state.gs_141784 = s_210_0;
        // N s_210_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #1u : u8
        let s_211_0: bool = true;
        // D s_211_1: write-var gs#141781 <= s_211_0
        fn_state.gs_141781 = s_211_0;
        // N s_211_2: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #1u : u8
        let s_212_0: bool = true;
        // D s_212_1: write-var gs#141782 <= s_212_0
        fn_state.gs_141782 = s_212_0;
        // N s_212_2: jump b209
        return block_209(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #1u : u8
        let s_213_0: bool = true;
        // D s_213_1: write-var gs#141783 <= s_213_0
        fn_state.gs_141783 = s_213_0;
        // N s_213_2: jump b210
        return block_210(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var tid3_trap:u8
        let s_214_0: bool = fn_state.tid3_trap;
        // N s_214_1: branch s_214_0 b216 b215
        if s_214_0 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_215(state, tracer, fn_state);
        };
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #64s : i64
        let s_215_0: i64 = 64;
        // C s_215_1: const #64s : i
        let s_215_1: i128 = 64;
        // S s_215_2: call Zeros(s_215_1)
        let s_215_2: Bits = Zeros(state, tracer, s_215_1);
        // S s_215_3: cast reint s_215_2 -> u64
        let s_215_3: u64 = (s_215_2.value() as u64);
        // S s_215_4: cast zx s_215_3 -> bv
        let s_215_4: Bits = Bits::new(s_215_3 as u128, 64u16);
        // D s_215_5: read-var t:i
        let s_215_5: i128 = fn_state.t;
        // D s_215_6: call X_set(s_215_5, s_215_0, s_215_4)
        let s_215_6: () = X_set(state, tracer, s_215_5, s_215_0, s_215_4);
        // N s_215_7: return
        return;
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #24u : u8
        let s_216_0: u8 = 24;
        // C s_216_1: cast zx s_216_0 -> bv
        let s_216_1: Bits = Bits::new(s_216_0 as u128, 8u16);
        // C s_216_2: cast zx s_216_1 -> i
        let s_216_2: i128 = (s_216_1.value() as i128);
        // C s_216_3: cast reint s_216_2 -> i64
        let s_216_3: i64 = (s_216_2 as i64);
        // C s_216_4: cast zx s_216_3 -> i
        let s_216_4: i128 = (i128::try_from(s_216_3).unwrap());
        // C s_216_5: const #432u : u32
        let s_216_5: u32 = 432;
        // D s_216_6: read-reg s_216_5:u8
        let s_216_6: u8 = {
            let value = state.read_register::<u8>(s_216_5 as isize);
            tracer.read_register(s_216_5 as isize, value);
            value
        };
        // D s_216_7: call AArch64_SystemAccessTrap(s_216_6, s_216_4)
        let s_216_7: () = AArch64_SystemAccessTrap(state, tracer, s_216_6, s_216_4);
        // N s_216_8: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_217_0: read-var op2:u8
        let s_217_0: u8 = fn_state.op2;
        // D s_217_1: cast zx s_217_0 -> bv
        let s_217_1: Bits = Bits::new(s_217_0 as u128, 3u16);
        // C s_217_2: const #2u : u8
        let s_217_2: u8 = 2;
        // C s_217_3: cast zx s_217_2 -> bv
        let s_217_3: Bits = Bits::new(s_217_2 as u128, 3u16);
        // D s_217_4: cmp-eq s_217_1 s_217_3
        let s_217_4: bool = ((s_217_1) == (s_217_3));
        // N s_217_5: branch s_217_4 b229 b218
        if s_217_4 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_218(state, tracer, fn_state);
        };
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var op2:u8
        let s_218_0: u8 = fn_state.op2;
        // D s_218_1: cast zx s_218_0 -> bv
        let s_218_1: Bits = Bits::new(s_218_0 as u128, 3u16);
        // C s_218_2: const #3u : u8
        let s_218_2: u8 = 3;
        // C s_218_3: cast zx s_218_2 -> bv
        let s_218_3: Bits = Bits::new(s_218_2 as u128, 3u16);
        // D s_218_4: cmp-eq s_218_1 s_218_3
        let s_218_4: bool = ((s_218_1) == (s_218_3));
        // N s_218_5: branch s_218_4 b228 b219
        if s_218_4 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_219_0: read-var op2:u8
        let s_219_0: u8 = fn_state.op2;
        // D s_219_1: cast zx s_219_0 -> bv
        let s_219_1: Bits = Bits::new(s_219_0 as u128, 3u16);
        // C s_219_2: const #5u : u8
        let s_219_2: u8 = 5;
        // C s_219_3: cast zx s_219_2 -> bv
        let s_219_3: Bits = Bits::new(s_219_2 as u128, 3u16);
        // D s_219_4: cmp-eq s_219_1 s_219_3
        let s_219_4: bool = ((s_219_1) == (s_219_3));
        // N s_219_5: branch s_219_4 b227 b220
        if s_219_4 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_220(state, tracer, fn_state);
        };
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var op2:u8
        let s_220_0: u8 = fn_state.op2;
        // D s_220_1: cast zx s_220_0 -> bv
        let s_220_1: Bits = Bits::new(s_220_0 as u128, 3u16);
        // C s_220_2: const #6u : u8
        let s_220_2: u8 = 6;
        // C s_220_3: cast zx s_220_2 -> bv
        let s_220_3: Bits = Bits::new(s_220_2 as u128, 3u16);
        // D s_220_4: cmp-eq s_220_1 s_220_3
        let s_220_4: bool = ((s_220_1) == (s_220_3));
        // N s_220_5: branch s_220_4 b226 b221
        if s_220_4 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_221(state, tracer, fn_state);
        };
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_221_0: read-var op2:u8
        let s_221_0: u8 = fn_state.op2;
        // D s_221_1: cast zx s_221_0 -> bv
        let s_221_1: Bits = Bits::new(s_221_0 as u128, 3u16);
        // C s_221_2: const #7u : u8
        let s_221_2: u8 = 7;
        // C s_221_3: cast zx s_221_2 -> bv
        let s_221_3: Bits = Bits::new(s_221_2 as u128, 3u16);
        // D s_221_4: cmp-eq s_221_1 s_221_3
        let s_221_4: bool = ((s_221_1) == (s_221_3));
        // D s_221_5: write-var gs#141776 <= s_221_4
        fn_state.gs_141776 = s_221_4;
        // N s_221_6: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_222_0: read-var gs#141776:u8
        let s_222_0: bool = fn_state.gs_141776;
        // D s_222_1: write-var gs#141777 <= s_222_0
        fn_state.gs_141777 = s_222_0;
        // N s_222_2: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var gs#141777:u8
        let s_223_0: bool = fn_state.gs_141777;
        // D s_223_1: write-var gs#141778 <= s_223_0
        fn_state.gs_141778 = s_223_0;
        // N s_223_2: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var gs#141778:u8
        let s_224_0: bool = fn_state.gs_141778;
        // D s_224_1: write-var gs#141779 <= s_224_0
        fn_state.gs_141779 = s_224_0;
        // N s_224_2: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_225_0: read-var gs#141779:u8
        let s_225_0: bool = fn_state.gs_141779;
        // D s_225_1: write-var gs#141780 <= s_225_0
        fn_state.gs_141780 = s_225_0;
        // N s_225_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #1u : u8
        let s_226_0: bool = true;
        // D s_226_1: write-var gs#141776 <= s_226_0
        fn_state.gs_141776 = s_226_0;
        // N s_226_2: jump b222
        return block_222(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #1u : u8
        let s_227_0: bool = true;
        // D s_227_1: write-var gs#141777 <= s_227_0
        fn_state.gs_141777 = s_227_0;
        // N s_227_2: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #1u : u8
        let s_228_0: bool = true;
        // D s_228_1: write-var gs#141778 <= s_228_0
        fn_state.gs_141778 = s_228_0;
        // N s_228_2: jump b224
        return block_224(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #1u : u8
        let s_229_0: bool = true;
        // D s_229_1: write-var gs#141779 <= s_229_0
        fn_state.gs_141779 = s_229_0;
        // N s_229_2: jump b225
        return block_225(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var tid3_trap:u8
        let s_230_0: bool = fn_state.tid3_trap;
        // N s_230_1: branch s_230_0 b232 b231
        if s_230_0 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_231(state, tracer, fn_state);
        };
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #64s : i64
        let s_231_0: i64 = 64;
        // C s_231_1: const #64s : i
        let s_231_1: i128 = 64;
        // S s_231_2: call Zeros(s_231_1)
        let s_231_2: Bits = Zeros(state, tracer, s_231_1);
        // S s_231_3: cast reint s_231_2 -> u64
        let s_231_3: u64 = (s_231_2.value() as u64);
        // S s_231_4: cast zx s_231_3 -> bv
        let s_231_4: Bits = Bits::new(s_231_3 as u128, 64u16);
        // D s_231_5: read-var t:i
        let s_231_5: i128 = fn_state.t;
        // D s_231_6: call X_set(s_231_5, s_231_0, s_231_4)
        let s_231_6: () = X_set(state, tracer, s_231_5, s_231_0, s_231_4);
        // N s_231_7: return
        return;
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #24u : u8
        let s_232_0: u8 = 24;
        // C s_232_1: cast zx s_232_0 -> bv
        let s_232_1: Bits = Bits::new(s_232_0 as u128, 8u16);
        // C s_232_2: cast zx s_232_1 -> i
        let s_232_2: i128 = (s_232_1.value() as i128);
        // C s_232_3: cast reint s_232_2 -> i64
        let s_232_3: i64 = (s_232_2 as i64);
        // C s_232_4: cast zx s_232_3 -> i
        let s_232_4: i128 = (i128::try_from(s_232_3).unwrap());
        // C s_232_5: const #432u : u32
        let s_232_5: u32 = 432;
        // D s_232_6: read-reg s_232_5:u8
        let s_232_6: u8 = {
            let value = state.read_register::<u8>(s_232_5 as isize);
            tracer.read_register(s_232_5 as isize, value);
            value
        };
        // D s_232_7: call AArch64_SystemAccessTrap(s_232_6, s_232_4)
        let s_232_7: () = AArch64_SystemAccessTrap(state, tracer, s_232_6, s_232_4);
        // N s_232_8: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_233_0: read-var op2:u8
        let s_233_0: u8 = fn_state.op2;
        // D s_233_1: cast zx s_233_0 -> bv
        let s_233_1: Bits = Bits::new(s_233_0 as u128, 3u16);
        // C s_233_2: const #3u : u8
        let s_233_2: u8 = 3;
        // C s_233_3: cast zx s_233_2 -> bv
        let s_233_3: Bits = Bits::new(s_233_2 as u128, 3u16);
        // D s_233_4: cmp-eq s_233_1 s_233_3
        let s_233_4: bool = ((s_233_1) == (s_233_3));
        // N s_233_5: branch s_233_4 b242 b234
        if s_233_4 {
            return block_242(state, tracer, fn_state);
        } else {
            return block_234(state, tracer, fn_state);
        };
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_234_0: read-var op2:u8
        let s_234_0: u8 = fn_state.op2;
        // D s_234_1: cast zx s_234_0 -> bv
        let s_234_1: Bits = Bits::new(s_234_0 as u128, 3u16);
        // C s_234_2: const #5u : u8
        let s_234_2: u8 = 5;
        // C s_234_3: cast zx s_234_2 -> bv
        let s_234_3: Bits = Bits::new(s_234_2 as u128, 3u16);
        // D s_234_4: cmp-eq s_234_1 s_234_3
        let s_234_4: bool = ((s_234_1) == (s_234_3));
        // N s_234_5: branch s_234_4 b241 b235
        if s_234_4 {
            return block_241(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_235_0: read-var op2:u8
        let s_235_0: u8 = fn_state.op2;
        // D s_235_1: cast zx s_235_0 -> bv
        let s_235_1: Bits = Bits::new(s_235_0 as u128, 3u16);
        // C s_235_2: const #6u : u8
        let s_235_2: u8 = 6;
        // C s_235_3: cast zx s_235_2 -> bv
        let s_235_3: Bits = Bits::new(s_235_2 as u128, 3u16);
        // D s_235_4: cmp-eq s_235_1 s_235_3
        let s_235_4: bool = ((s_235_1) == (s_235_3));
        // N s_235_5: branch s_235_4 b240 b236
        if s_235_4 {
            return block_240(state, tracer, fn_state);
        } else {
            return block_236(state, tracer, fn_state);
        };
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var op2:u8
        let s_236_0: u8 = fn_state.op2;
        // D s_236_1: cast zx s_236_0 -> bv
        let s_236_1: Bits = Bits::new(s_236_0 as u128, 3u16);
        // C s_236_2: const #7u : u8
        let s_236_2: u8 = 7;
        // C s_236_3: cast zx s_236_2 -> bv
        let s_236_3: Bits = Bits::new(s_236_2 as u128, 3u16);
        // D s_236_4: cmp-eq s_236_1 s_236_3
        let s_236_4: bool = ((s_236_1) == (s_236_3));
        // D s_236_5: write-var gs#141772 <= s_236_4
        fn_state.gs_141772 = s_236_4;
        // N s_236_6: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_237_0: read-var gs#141772:u8
        let s_237_0: bool = fn_state.gs_141772;
        // D s_237_1: write-var gs#141773 <= s_237_0
        fn_state.gs_141773 = s_237_0;
        // N s_237_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var gs#141773:u8
        let s_238_0: bool = fn_state.gs_141773;
        // D s_238_1: write-var gs#141774 <= s_238_0
        fn_state.gs_141774 = s_238_0;
        // N s_238_2: jump b239
        return block_239(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_239_0: read-var gs#141774:u8
        let s_239_0: bool = fn_state.gs_141774;
        // D s_239_1: write-var gs#141775 <= s_239_0
        fn_state.gs_141775 = s_239_0;
        // N s_239_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_240_0: const #1u : u8
        let s_240_0: bool = true;
        // D s_240_1: write-var gs#141772 <= s_240_0
        fn_state.gs_141772 = s_240_0;
        // N s_240_2: jump b237
        return block_237(state, tracer, fn_state);
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #1u : u8
        let s_241_0: bool = true;
        // D s_241_1: write-var gs#141773 <= s_241_0
        fn_state.gs_141773 = s_241_0;
        // N s_241_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_242_0: const #1u : u8
        let s_242_0: bool = true;
        // D s_242_1: write-var gs#141774 <= s_242_0
        fn_state.gs_141774 = s_242_0;
        // N s_242_2: jump b239
        return block_239(state, tracer, fn_state);
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_243_0: read-var tid3_trap:u8
        let s_243_0: bool = fn_state.tid3_trap;
        // N s_243_1: branch s_243_0 b245 b244
        if s_243_0 {
            return block_245(state, tracer, fn_state);
        } else {
            return block_244(state, tracer, fn_state);
        };
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_244_0: const #64s : i64
        let s_244_0: i64 = 64;
        // C s_244_1: const #64s : i
        let s_244_1: i128 = 64;
        // S s_244_2: call Zeros(s_244_1)
        let s_244_2: Bits = Zeros(state, tracer, s_244_1);
        // S s_244_3: cast reint s_244_2 -> u64
        let s_244_3: u64 = (s_244_2.value() as u64);
        // S s_244_4: cast zx s_244_3 -> bv
        let s_244_4: Bits = Bits::new(s_244_3 as u128, 64u16);
        // D s_244_5: read-var t:i
        let s_244_5: i128 = fn_state.t;
        // D s_244_6: call X_set(s_244_5, s_244_0, s_244_4)
        let s_244_6: () = X_set(state, tracer, s_244_5, s_244_0, s_244_4);
        // N s_244_7: return
        return;
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #24u : u8
        let s_245_0: u8 = 24;
        // C s_245_1: cast zx s_245_0 -> bv
        let s_245_1: Bits = Bits::new(s_245_0 as u128, 8u16);
        // C s_245_2: cast zx s_245_1 -> i
        let s_245_2: i128 = (s_245_1.value() as i128);
        // C s_245_3: cast reint s_245_2 -> i64
        let s_245_3: i64 = (s_245_2 as i64);
        // C s_245_4: cast zx s_245_3 -> i
        let s_245_4: i128 = (i128::try_from(s_245_3).unwrap());
        // C s_245_5: const #432u : u32
        let s_245_5: u32 = 432;
        // D s_245_6: read-reg s_245_5:u8
        let s_245_6: u8 = {
            let value = state.read_register::<u8>(s_245_5 as isize);
            tracer.read_register(s_245_5 as isize, value);
            value
        };
        // D s_245_7: call AArch64_SystemAccessTrap(s_245_6, s_245_4)
        let s_245_7: () = AArch64_SystemAccessTrap(state, tracer, s_245_6, s_245_4);
        // N s_245_8: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_246_0: read-var op2:u8
        let s_246_0: u8 = fn_state.op2;
        // D s_246_1: cast zx s_246_0 -> bv
        let s_246_1: Bits = Bits::new(s_246_0 as u128, 3u16);
        // C s_246_2: const #7u : u8
        let s_246_2: u8 = 7;
        // C s_246_3: cast zx s_246_2 -> bv
        let s_246_3: Bits = Bits::new(s_246_2 as u128, 3u16);
        // D s_246_4: cmp-eq s_246_1 s_246_3
        let s_246_4: bool = ((s_246_1) == (s_246_3));
        // D s_246_5: write-var gs#141771 <= s_246_4
        fn_state.gs_141771 = s_246_4;
        // N s_246_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_247_0: read-var crn:u8
        let s_247_0: u8 = fn_state.crn;
        // D s_247_1: cast zx s_247_0 -> bv
        let s_247_1: Bits = Bits::new(s_247_0 as u128, 4u16);
        // C s_247_2: const #0u : u8
        let s_247_2: u8 = 0;
        // C s_247_3: cast zx s_247_2 -> bv
        let s_247_3: Bits = Bits::new(s_247_2 as u128, 4u16);
        // D s_247_4: cmp-eq s_247_1 s_247_3
        let s_247_4: bool = ((s_247_1) == (s_247_3));
        // D s_247_5: write-var gs#141769 <= s_247_4
        fn_state.gs_141769 = s_247_4;
        // N s_247_6: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_248_0: read-var op1:u8
        let s_248_0: u8 = fn_state.op1;
        // D s_248_1: cast zx s_248_0 -> bv
        let s_248_1: Bits = Bits::new(s_248_0 as u128, 3u16);
        // C s_248_2: const #0u : u8
        let s_248_2: u8 = 0;
        // C s_248_3: cast zx s_248_2 -> bv
        let s_248_3: Bits = Bits::new(s_248_2 as u128, 3u16);
        // D s_248_4: cmp-eq s_248_1 s_248_3
        let s_248_4: bool = ((s_248_1) == (s_248_3));
        // D s_248_5: write-var gs#141768 <= s_248_4
        fn_state.gs_141768 = s_248_4;
        // N s_248_6: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_249_0: read-var read:u8
        let s_249_0: bool = fn_state.read;
        // D s_249_1: write-var gs#141766 <= s_249_0
        fn_state.gs_141766 = s_249_0;
        // N s_249_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_250_0: const #1u : u8
        let s_250_0: bool = true;
        // D s_250_1: write-var gs#141764 <= s_250_0
        fn_state.gs_141764 = s_250_0;
        // N s_250_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_251_0: const #1u : u8
        let s_251_0: bool = true;
        // D s_251_1: write-var gs#141765 <= s_251_0
        fn_state.gs_141765 = s_251_0;
        // N s_251_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_252_0: const #() : ()
        let s_252_0: () = ();
        // S s_252_1: call HaveFGTExt(s_252_0)
        let s_252_1: bool = HaveFGTExt(state, tracer, s_252_0);
        // N s_252_2: branch s_252_1 b255 b253
        if s_252_1 {
            return block_255(state, tracer, fn_state);
        } else {
            return block_253(state, tracer, fn_state);
        };
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_253_0: const #"Unallocated encodings trapped by HCR_EL2.TID3" : str
        let s_253_0: &'static str = "Unallocated encodings trapped by HCR_EL2.TID3";
        // S s_253_1: call __IMPDEF_boolean(s_253_0)
        let s_253_1: bool = u__IMPDEF_boolean(state, tracer, s_253_0);
        // D s_253_2: write-var tid3_trap <= s_253_1
        fn_state.tid3_trap = s_253_1;
        // N s_253_3: jump b254
        return block_254(state, tracer, fn_state);
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_254_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_255_0: const #1u : u8
        let s_255_0: bool = true;
        // D s_255_1: write-var tid3_trap <= s_255_0
        fn_state.tid3_trap = s_255_0;
        // N s_255_2: jump b254
        return block_254(state, tracer, fn_state);
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_256_0: const #102552u : u32
        let s_256_0: u32 = 102552;
        // D s_256_1: read-reg s_256_0:struct
        let s_256_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_256_0 as isize);
            tracer.read_register(s_256_0 as isize, value);
            value
        };
        // D s_256_2: call _get_HCR_EL2_Type_TID3(s_256_1)
        let s_256_2: bool = u_get_HCR_EL2_Type_TID3(state, tracer, s_256_1);
        // D s_256_3: cast zx s_256_2 -> bv
        let s_256_3: Bits = Bits::new(s_256_2 as u128, 1u16);
        // C s_256_4: const #1u : u8
        let s_256_4: bool = true;
        // C s_256_5: cast zx s_256_4 -> bv
        let s_256_5: Bits = Bits::new(s_256_4 as u128, 1u16);
        // D s_256_6: cmp-eq s_256_3 s_256_5
        let s_256_6: bool = ((s_256_3) == (s_256_5));
        // D s_256_7: write-var gs#141763 <= s_256_6
        fn_state.gs_141763 = s_256_6;
        // N s_256_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #() : ()
        let s_257_0: () = ();
        // S s_257_1: call EL2Enabled(s_257_0)
        let s_257_1: bool = EL2Enabled(state, tracer, s_257_0);
        // D s_257_2: write-var gs#141762 <= s_257_1
        fn_state.gs_141762 = s_257_1;
        // N s_257_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var read:u8
        let s_258_0: bool = fn_state.read;
        // D s_258_1: write-var gs#141761 <= s_258_0
        fn_state.gs_141761 = s_258_0;
        // N s_258_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
