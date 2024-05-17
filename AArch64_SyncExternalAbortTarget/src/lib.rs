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
use HaveDoubleFault2Ext::*;
use IsInHost::*;
use u_get_SCR_EL3_Type_TMEA::*;
use IsHCRXEL2Enabled::*;
use IsSecondStage::*;
use u_get_HCR_EL2_Type_TEA::*;
use EffectiveEA::*;
use EL2Enabled::*;
use HaveRASExt::*;
use u_get_HCRX_EL2_Type_TMEA::*;
use u_get_HCR_EL2_Type_TGE::*;
use common::*;
pub fn AArch64_SyncExternalAbortTarget<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        route_to_el2: bool,
        take_in_el1_0: bool,
        gs_9807: bool,
        gs_9799: bool,
        tea_bit: bool,
        gs_9793: bool,
        gs_9818: bool,
        gs_9801: bool,
        take_in_el2_0: bool,
        gs_9816: bool,
        gs_9820: bool,
        gs_9792: bool,
        gs_9808: bool,
        gs_9786: bool,
        gs_9803: bool,
        gs_9811: bool,
        gs_9790: bool,
        ga_6994: ProductType9878976b5bcce9c9,
        route_masked_to_el2: bool,
        gs_9821: bool,
        take_in_el3: bool,
        gs_9795: bool,
        gs_9810: bool,
        gs_9817: bool,
        target_el: u8,
        gs_9814: bool,
        gs_9819: bool,
        route_masked_to_el3: bool,
        gs_9805: bool,
        gs_9824: bool,
        gs_9794: bool,
        gs_9788: bool,
        gs_9812: bool,
        gs_9813: bool,
        route_to_el3: bool,
        gs_9787: bool,
        gs_9800: bool,
        gs_9809: bool,
        gs_9806: bool,
        gs_9802: bool,
        gs_9823: bool,
        gs_9815: bool,
        gs_9791: bool,
        gs_9796: bool,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #424u : u32
        let s_0_3: u32 = 424;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-ne s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) != (s_0_5));
        // N s_0_7: branch s_0_6 b120 b1
        if s_0_6 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var route_to_el3 <= s_1_0
        fn_state.route_to_el3 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveRASExt(s_2_0)
        let s_2_1: bool = HaveRASExt(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b119 b3
        if s_2_1 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var tea_bit <= s_3_0
        fn_state.tea_bit = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var route_to_el3:u8
        let s_4_0: bool = fn_state.route_to_el3;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b118 b5
        if s_4_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#9787 <= s_5_0
        fn_state.gs_9787 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_6_0: read-var gs#9787:u8
        let s_6_0: bool = fn_state.gs_9787;
        // N s_6_1: branch s_6_0 b117 b7
        if s_6_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#9788 <= s_7_0
        fn_state.gs_9788 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_8_0: read-var gs#9788:u8
        let s_8_0: bool = fn_state.gs_9788;
        // N s_8_1: branch s_8_0 b110 b9
        if s_8_0 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_9_0: read-var route_to_el3:u8
        let s_9_0: bool = fn_state.route_to_el3;
        // D s_9_1: not s_9_0
        let s_9_1: bool = !s_9_0;
        // N s_9_2: branch s_9_1 b109 b10
        if s_9_1 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#9790 <= s_10_0
        fn_state.gs_9790 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_11_0: read-var gs#9790:u8
        let s_11_0: bool = fn_state.gs_9790;
        // N s_11_1: branch s_11_0 b108 b12
        if s_11_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#9791 <= s_12_0
        fn_state.gs_9791 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_13_0: read-var gs#9791:u8
        let s_13_0: bool = fn_state.gs_9791;
        // N s_13_1: branch s_13_0 b98 b14
        if s_13_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var route_to_el2 <= s_14_0
        fn_state.route_to_el2 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveDoubleFault2Ext(s_15_0)
        let s_15_1: bool = HaveDoubleFault2Ext(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b64 b16
        if s_15_1 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var route_masked_to_el2 <= s_16_0
        fn_state.route_masked_to_el2 = s_16_0;
        // C s_16_2: const #0u : u8
        let s_16_2: bool = false;
        // D s_16_3: write-var route_masked_to_el3 <= s_16_2
        fn_state.route_masked_to_el3 = s_16_2;
        // N s_16_4: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #16975u : u32
        let s_17_0: u32 = 16975;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 2u16);
        // C s_17_3: const #424u : u32
        let s_17_3: u32 = 424;
        // D s_17_4: read-reg s_17_3:u8
        let s_17_4: u8 = {
            let value = state.read_register::<u8>(s_17_3 as isize);
            tracer.read_register(s_17_3 as isize, value);
            value
        };
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 2u16);
        // D s_17_6: cmp-eq s_17_2 s_17_5
        let s_17_6: bool = ((s_17_2) == (s_17_5));
        // D s_17_7: write-var take_in_el3 <= s_17_6
        fn_state.take_in_el3 = s_17_6;
        // C s_17_8: const #16975u : u32
        let s_17_8: u32 = 16975;
        // D s_17_9: read-reg s_17_8:u8
        let s_17_9: u8 = {
            let value = state.read_register::<u8>(s_17_8 as isize);
            tracer.read_register(s_17_8 as isize, value);
            value
        };
        // D s_17_10: cast zx s_17_9 -> bv
        let s_17_10: Bits = Bits::new(s_17_9 as u128, 2u16);
        // C s_17_11: const #432u : u32
        let s_17_11: u32 = 432;
        // D s_17_12: read-reg s_17_11:u8
        let s_17_12: u8 = {
            let value = state.read_register::<u8>(s_17_11 as isize);
            tracer.read_register(s_17_11 as isize, value);
            value
        };
        // D s_17_13: cast zx s_17_12 -> bv
        let s_17_13: Bits = Bits::new(s_17_12 as u128, 2u16);
        // D s_17_14: cmp-eq s_17_10 s_17_13
        let s_17_14: bool = ((s_17_10) == (s_17_13));
        // N s_17_15: branch s_17_14 b63 b18
        if s_17_14 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call IsInHost(s_18_0)
        let s_18_1: bool = IsInHost(state, tracer, s_18_0);
        // D s_18_2: write-var gs#9811 <= s_18_1
        fn_state.gs_9811 = s_18_1;
        // N s_18_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_19_0: read-var gs#9811:u8
        let s_19_0: bool = fn_state.gs_9811;
        // N s_19_1: branch s_19_0 b59 b20
        if s_19_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#9813 <= s_20_0
        fn_state.gs_9813 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_21_0: read-var gs#9813:u8
        let s_21_0: bool = fn_state.gs_9813;
        // D s_21_1: write-var take_in_el2_0 <= s_21_0
        fn_state.take_in_el2_0 = s_21_0;
        // C s_21_2: const #16975u : u32
        let s_21_2: u32 = 16975;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: u8 = {
            let value = state.read_register::<u8>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // D s_21_4: cast zx s_21_3 -> bv
        let s_21_4: Bits = Bits::new(s_21_3 as u128, 2u16);
        // C s_21_5: const #440u : u32
        let s_21_5: u32 = 440;
        // D s_21_6: read-reg s_21_5:u8
        let s_21_6: u8 = {
            let value = state.read_register::<u8>(s_21_5 as isize);
            tracer.read_register(s_21_5 as isize, value);
            value
        };
        // D s_21_7: cast zx s_21_6 -> bv
        let s_21_7: Bits = Bits::new(s_21_6 as u128, 2u16);
        // D s_21_8: cmp-eq s_21_4 s_21_7
        let s_21_8: bool = ((s_21_4) == (s_21_7));
        // N s_21_9: branch s_21_8 b58 b22
        if s_21_8 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_22_0: const #16975u : u32
        let s_22_0: u32 = 16975;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 2u16);
        // C s_22_3: const #448u : u32
        let s_22_3: u32 = 448;
        // D s_22_4: read-reg s_22_3:u8
        let s_22_4: u8 = {
            let value = state.read_register::<u8>(s_22_3 as isize);
            tracer.read_register(s_22_3 as isize, value);
            value
        };
        // D s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 2u16);
        // D s_22_6: cmp-eq s_22_2 s_22_5
        let s_22_6: bool = ((s_22_2) == (s_22_5));
        // N s_22_7: branch s_22_6 b57 b23
        if s_22_6 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#9814 <= s_23_0
        fn_state.gs_9814 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_24_0: read-var gs#9814:u8
        let s_24_0: bool = fn_state.gs_9814;
        // D s_24_1: write-var gs#9815 <= s_24_0
        fn_state.gs_9815 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_25_0: read-var gs#9815:u8
        let s_25_0: bool = fn_state.gs_9815;
        // N s_25_1: branch s_25_0 b53 b26
        if s_25_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#9817 <= s_26_0
        fn_state.gs_9817 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_27_0: read-var gs#9817:u8
        let s_27_0: bool = fn_state.gs_9817;
        // N s_27_1: branch s_27_0 b49 b28
        if s_27_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#9819 <= s_28_0
        fn_state.gs_9819 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_29_0: read-var gs#9819:u8
        let s_29_0: bool = fn_state.gs_9819;
        // D s_29_1: write-var take_in_el1_0 <= s_29_0
        fn_state.take_in_el1_0 = s_29_0;
        // D s_29_2: read-var take_in_el3:u8
        let s_29_2: bool = fn_state.take_in_el3;
        // N s_29_3: branch s_29_2 b48 b30
        if s_29_2 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_30_0: read-var route_to_el3:u8
        let s_30_0: bool = fn_state.route_to_el3;
        // D s_30_1: write-var gs#9820 <= s_30_0
        fn_state.gs_9820 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_31_0: read-var gs#9820:u8
        let s_31_0: bool = fn_state.gs_9820;
        // N s_31_1: branch s_31_0 b47 b32
        if s_31_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_32_0: read-var route_masked_to_el3:u8
        let s_32_0: bool = fn_state.route_masked_to_el3;
        // D s_32_1: write-var gs#9821 <= s_32_0
        fn_state.gs_9821 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_33_0: read-var gs#9821:u8
        let s_33_0: bool = fn_state.gs_9821;
        // N s_33_1: branch s_33_0 b46 b34
        if s_33_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_34_0: read-var take_in_el2_0:u8
        let s_34_0: bool = fn_state.take_in_el2_0;
        // N s_34_1: branch s_34_0 b45 b35
        if s_34_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_35_0: read-var route_to_el2:u8
        let s_35_0: bool = fn_state.route_to_el2;
        // D s_35_1: write-var gs#9823 <= s_35_0
        fn_state.gs_9823 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_36_0: read-var gs#9823:u8
        let s_36_0: bool = fn_state.gs_9823;
        // N s_36_1: branch s_36_0 b44 b37
        if s_36_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_37_0: read-var route_masked_to_el2:u8
        let s_37_0: bool = fn_state.route_masked_to_el2;
        // D s_37_1: write-var gs#9824 <= s_37_0
        fn_state.gs_9824 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_38_0: read-var gs#9824:u8
        let s_38_0: bool = fn_state.gs_9824;
        // N s_38_1: branch s_38_0 b43 b39
        if s_38_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_39_0: read-var take_in_el1_0:u8
        let s_39_0: bool = fn_state.take_in_el1_0;
        // N s_39_1: branch s_39_0 b42 b40
        if s_39_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // N s_40_1: assert s_40_0
        let s_40_1: () = assert!(s_40_0);
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_41_0: read-var target_el:u8
        let s_41_0: u8 = fn_state.target_el;
        // N s_41_1: return s_41_0
        return s_41_0;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_42_0: const #440u : u32
        let s_42_0: u32 = 440;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: write-var target_el <= s_42_1
        fn_state.target_el = s_42_1;
        // N s_42_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_43_0: const #432u : u32
        let s_43_0: u32 = 432;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: write-var target_el <= s_43_1
        fn_state.target_el = s_43_1;
        // N s_43_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#9824 <= s_44_0
        fn_state.gs_9824 = s_44_0;
        // N s_44_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#9823 <= s_45_0
        fn_state.gs_9823 = s_45_0;
        // N s_45_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_46_0: const #424u : u32
        let s_46_0: u32 = 424;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: write-var target_el <= s_46_1
        fn_state.target_el = s_46_1;
        // N s_46_3: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#9821 <= s_47_0
        fn_state.gs_9821 = s_47_0;
        // N s_47_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#9820 <= s_48_0
        fn_state.gs_9820 = s_48_0;
        // N s_48_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_49_0: read-var route_to_el3:u8
        let s_49_0: bool = fn_state.route_to_el3;
        // N s_49_1: branch s_49_0 b52 b50
        if s_49_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_50_0: read-var route_masked_to_el3:u8
        let s_50_0: bool = fn_state.route_masked_to_el3;
        // D s_50_1: write-var gs#9818 <= s_50_0
        fn_state.gs_9818 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_51_0: read-var gs#9818:u8
        let s_51_0: bool = fn_state.gs_9818;
        // D s_51_1: not s_51_0
        let s_51_1: bool = !s_51_0;
        // D s_51_2: write-var gs#9819 <= s_51_1
        fn_state.gs_9819 = s_51_1;
        // N s_51_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#9818 <= s_52_0
        fn_state.gs_9818 = s_52_0;
        // N s_52_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_53_0: read-var route_to_el2:u8
        let s_53_0: bool = fn_state.route_to_el2;
        // N s_53_1: branch s_53_0 b56 b54
        if s_53_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_54_0: read-var route_masked_to_el2:u8
        let s_54_0: bool = fn_state.route_masked_to_el2;
        // D s_54_1: write-var gs#9816 <= s_54_0
        fn_state.gs_9816 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_55_0: read-var gs#9816:u8
        let s_55_0: bool = fn_state.gs_9816;
        // D s_55_1: not s_55_0
        let s_55_1: bool = !s_55_0;
        // D s_55_2: write-var gs#9817 <= s_55_1
        fn_state.gs_9817 = s_55_1;
        // N s_55_3: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_56_0: const #1u : u8
        let s_56_0: bool = true;
        // D s_56_1: write-var gs#9816 <= s_56_0
        fn_state.gs_9816 = s_56_0;
        // N s_56_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call IsInHost(s_57_0)
        let s_57_1: bool = IsInHost(state, tracer, s_57_0);
        // S s_57_2: not s_57_1
        let s_57_2: bool = !s_57_1;
        // D s_57_3: write-var gs#9814 <= s_57_2
        fn_state.gs_9814 = s_57_2;
        // N s_57_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#9815 <= s_58_0
        fn_state.gs_9815 = s_58_0;
        // N s_58_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_59_0: read-var route_to_el3:u8
        let s_59_0: bool = fn_state.route_to_el3;
        // N s_59_1: branch s_59_0 b62 b60
        if s_59_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_60_0: read-var route_masked_to_el3:u8
        let s_60_0: bool = fn_state.route_masked_to_el3;
        // D s_60_1: write-var gs#9812 <= s_60_0
        fn_state.gs_9812 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_61_0: read-var gs#9812:u8
        let s_61_0: bool = fn_state.gs_9812;
        // D s_61_1: not s_61_0
        let s_61_1: bool = !s_61_0;
        // D s_61_2: write-var gs#9813 <= s_61_1
        fn_state.gs_9813 = s_61_1;
        // N s_61_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_62_0: const #1u : u8
        let s_62_0: bool = true;
        // D s_62_1: write-var gs#9812 <= s_62_0
        fn_state.gs_9812 = s_62_0;
        // N s_62_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#9811 <= s_63_0
        fn_state.gs_9811 = s_63_0;
        // N s_63_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call EL2Enabled(s_64_0)
        let s_64_1: bool = EL2Enabled(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b97 b65
        if s_64_1 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#9799 <= s_65_0
        fn_state.gs_9799 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_66_0: read-var gs#9799:u8
        let s_66_0: bool = fn_state.gs_9799;
        // N s_66_1: branch s_66_0 b93 b67
        if s_66_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#9801 <= s_67_0
        fn_state.gs_9801 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_68_0: read-var gs#9801:u8
        let s_68_0: bool = fn_state.gs_9801;
        // N s_68_1: branch s_68_0 b92 b69
        if s_68_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#9802 <= s_69_0
        fn_state.gs_9802 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_70_0: read-var gs#9802:u8
        let s_70_0: bool = fn_state.gs_9802;
        // N s_70_1: branch s_70_0 b91 b71
        if s_70_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#9803 <= s_71_0
        fn_state.gs_9803 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_72_0: read-var gs#9803:u8
        let s_72_0: bool = fn_state.gs_9803;
        // D s_72_1: write-var route_masked_to_el2 <= s_72_0
        fn_state.route_masked_to_el2 = s_72_0;
        // C s_72_2: const #424u : u32
        let s_72_2: u32 = 424;
        // D s_72_3: read-reg s_72_2:u8
        let s_72_3: u8 = {
            let value = state.read_register::<u8>(s_72_2 as isize);
            tracer.read_register(s_72_2 as isize, value);
            value
        };
        // C s_72_4: const #2u : u8
        let s_72_4: u8 = 2;
        // D s_72_5: cmp-lt s_72_3 s_72_4
        let s_72_5: bool = ((s_72_3) < (s_72_4));
        // N s_72_6: branch s_72_5 b87 b73
        if s_72_5 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#9806 <= s_73_0
        fn_state.gs_9806 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_74_0: read-var gs#9806:u8
        let s_74_0: bool = fn_state.gs_9806;
        // N s_74_1: branch s_74_0 b80 b75
        if s_74_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#9809 <= s_75_0
        fn_state.gs_9809 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_76_0: read-var gs#9809:u8
        let s_76_0: bool = fn_state.gs_9809;
        // N s_76_1: branch s_76_0 b79 b77
        if s_76_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#9810 <= s_77_0
        fn_state.gs_9810 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_78_0: read-var gs#9810:u8
        let s_78_0: bool = fn_state.gs_9810;
        // D s_78_1: write-var route_masked_to_el3 <= s_78_0
        fn_state.route_masked_to_el3 = s_78_0;
        // N s_78_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_79_0: const #90704u : u32
        let s_79_0: u32 = 90704;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_SCR_EL3_Type_TMEA(s_79_1)
        let s_79_2: bool = u_get_SCR_EL3_Type_TMEA(state, tracer, s_79_1);
        // D s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // C s_79_4: const #1u : u8
        let s_79_4: bool = true;
        // C s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 1u16);
        // D s_79_6: cmp-eq s_79_3 s_79_5
        let s_79_6: bool = ((s_79_3) == (s_79_5));
        // D s_79_7: write-var gs#9810 <= s_79_6
        fn_state.gs_9810 = s_79_6;
        // N s_79_8: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_80_0: const #16975u : u32
        let s_80_0: u32 = 16975;
        // D s_80_1: read-reg s_80_0:u8
        let s_80_1: u8 = {
            let value = state.read_register::<u8>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: cast zx s_80_1 -> bv
        let s_80_2: Bits = Bits::new(s_80_1 as u128, 2u16);
        // C s_80_3: const #432u : u32
        let s_80_3: u32 = 432;
        // D s_80_4: read-reg s_80_3:u8
        let s_80_4: u8 = {
            let value = state.read_register::<u8>(s_80_3 as isize);
            tracer.read_register(s_80_3 as isize, value);
            value
        };
        // D s_80_5: cast zx s_80_4 -> bv
        let s_80_5: Bits = Bits::new(s_80_4 as u128, 2u16);
        // D s_80_6: cmp-eq s_80_2 s_80_5
        let s_80_6: bool = ((s_80_2) == (s_80_5));
        // N s_80_7: branch s_80_6 b86 b81
        if s_80_6 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_81_0: const #16975u : u32
        let s_81_0: u32 = 16975;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: cast zx s_81_1 -> bv
        let s_81_2: Bits = Bits::new(s_81_1 as u128, 2u16);
        // C s_81_3: const #440u : u32
        let s_81_3: u32 = 440;
        // D s_81_4: read-reg s_81_3:u8
        let s_81_4: u8 = {
            let value = state.read_register::<u8>(s_81_3 as isize);
            tracer.read_register(s_81_3 as isize, value);
            value
        };
        // D s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 2u16);
        // D s_81_6: cmp-eq s_81_2 s_81_5
        let s_81_6: bool = ((s_81_2) == (s_81_5));
        // D s_81_7: write-var gs#9807 <= s_81_6
        fn_state.gs_9807 = s_81_6;
        // N s_81_8: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_82_0: read-var gs#9807:u8
        let s_82_0: bool = fn_state.gs_9807;
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
    ) -> u8 {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#9808 <= s_83_0
        fn_state.gs_9808 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_84_0: read-var gs#9808:u8
        let s_84_0: bool = fn_state.gs_9808;
        // D s_84_1: write-var gs#9809 <= s_84_0
        fn_state.gs_9809 = s_84_0;
        // N s_84_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_85_0: const #16968u : u32
        let s_85_0: u32 = 16968;
        // D s_85_1: read-reg s_85_0:u8
        let s_85_1: bool = {
            let value = state.read_register::<bool>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: cast zx s_85_1 -> bv
        let s_85_2: Bits = Bits::new(s_85_1 as u128, 1u16);
        // C s_85_3: const #1u : u8
        let s_85_3: bool = true;
        // C s_85_4: cast zx s_85_3 -> bv
        let s_85_4: Bits = Bits::new(s_85_3 as u128, 1u16);
        // D s_85_5: cmp-eq s_85_2 s_85_4
        let s_85_5: bool = ((s_85_2) == (s_85_4));
        // D s_85_6: write-var gs#9808 <= s_85_5
        fn_state.gs_9808 = s_85_5;
        // N s_85_7: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_86_0: const #1u : u8
        let s_86_0: bool = true;
        // D s_86_1: write-var gs#9807 <= s_86_0
        fn_state.gs_9807 = s_86_0;
        // N s_86_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_87_0: read-var route_to_el2:u8
        let s_87_0: bool = fn_state.route_to_el2;
        // N s_87_1: branch s_87_0 b90 b88
        if s_87_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_88_0: read-var route_masked_to_el2:u8
        let s_88_0: bool = fn_state.route_masked_to_el2;
        // D s_88_1: write-var gs#9805 <= s_88_0
        fn_state.gs_9805 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_89_0: read-var gs#9805:u8
        let s_89_0: bool = fn_state.gs_9805;
        // D s_89_1: not s_89_0
        let s_89_1: bool = !s_89_0;
        // D s_89_2: write-var gs#9806 <= s_89_1
        fn_state.gs_9806 = s_89_1;
        // N s_89_3: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_90_0: const #1u : u8
        let s_90_0: bool = true;
        // D s_90_1: write-var gs#9805 <= s_90_0
        fn_state.gs_9805 = s_90_0;
        // N s_90_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_91_0: const #22528u : u32
        let s_91_0: u32 = 22528;
        // D s_91_1: read-reg s_91_0:struct
        let s_91_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call _get_HCRX_EL2_Type_TMEA(s_91_1)
        let s_91_2: bool = u_get_HCRX_EL2_Type_TMEA(state, tracer, s_91_1);
        // D s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // C s_91_4: const #1u : u8
        let s_91_4: bool = true;
        // C s_91_5: cast zx s_91_4 -> bv
        let s_91_5: Bits = Bits::new(s_91_4 as u128, 1u16);
        // D s_91_6: cmp-eq s_91_3 s_91_5
        let s_91_6: bool = ((s_91_3) == (s_91_5));
        // D s_91_7: write-var gs#9803 <= s_91_6
        fn_state.gs_9803 = s_91_6;
        // N s_91_8: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_92_0: const #() : ()
        let s_92_0: () = ();
        // S s_92_1: call IsHCRXEL2Enabled(s_92_0)
        let s_92_1: bool = IsHCRXEL2Enabled(state, tracer, s_92_0);
        // D s_92_2: write-var gs#9802 <= s_92_1
        fn_state.gs_9802 = s_92_1;
        // N s_92_3: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_93_0: const #16975u : u32
        let s_93_0: u32 = 16975;
        // D s_93_1: read-reg s_93_0:u8
        let s_93_1: u8 = {
            let value = state.read_register::<u8>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: cast zx s_93_1 -> bv
        let s_93_2: Bits = Bits::new(s_93_1 as u128, 2u16);
        // C s_93_3: const #440u : u32
        let s_93_3: u32 = 440;
        // D s_93_4: read-reg s_93_3:u8
        let s_93_4: u8 = {
            let value = state.read_register::<u8>(s_93_3 as isize);
            tracer.read_register(s_93_3 as isize, value);
            value
        };
        // D s_93_5: cast zx s_93_4 -> bv
        let s_93_5: Bits = Bits::new(s_93_4 as u128, 2u16);
        // D s_93_6: cmp-eq s_93_2 s_93_5
        let s_93_6: bool = ((s_93_2) == (s_93_5));
        // N s_93_7: branch s_93_6 b96 b94
        if s_93_6 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#9800 <= s_94_0
        fn_state.gs_9800 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_95_0: read-var gs#9800:u8
        let s_95_0: bool = fn_state.gs_9800;
        // D s_95_1: write-var gs#9801 <= s_95_0
        fn_state.gs_9801 = s_95_0;
        // N s_95_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_96_0: const #16968u : u32
        let s_96_0: u32 = 16968;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: bool = {
            let value = state.read_register::<bool>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: cast zx s_96_1 -> bv
        let s_96_2: Bits = Bits::new(s_96_1 as u128, 1u16);
        // C s_96_3: const #1u : u8
        let s_96_3: bool = true;
        // C s_96_4: cast zx s_96_3 -> bv
        let s_96_4: Bits = Bits::new(s_96_3 as u128, 1u16);
        // D s_96_5: cmp-eq s_96_2 s_96_4
        let s_96_5: bool = ((s_96_2) == (s_96_4));
        // D s_96_6: write-var gs#9800 <= s_96_5
        fn_state.gs_9800 = s_96_5;
        // N s_96_7: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_97_0: read-var route_to_el3:u8
        let s_97_0: bool = fn_state.route_to_el3;
        // D s_97_1: not s_97_0
        let s_97_1: bool = !s_97_0;
        // D s_97_2: write-var gs#9799 <= s_97_1
        fn_state.gs_9799 = s_97_1;
        // N s_97_3: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call IsInHost(s_98_0)
        let s_98_1: bool = IsInHost(state, tracer, s_98_0);
        // S s_98_2: not s_98_1
        let s_98_2: bool = !s_98_1;
        // N s_98_3: branch s_98_2 b101 b99
        if s_98_2 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#9794 <= s_99_0
        fn_state.gs_9794 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_100_0: read-var gs#9794:u8
        let s_100_0: bool = fn_state.gs_9794;
        // D s_100_1: write-var route_to_el2 <= s_100_0
        fn_state.route_to_el2 = s_100_0;
        // N s_100_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_101_0: const #102552u : u32
        let s_101_0: u32 = 102552;
        // D s_101_1: read-reg s_101_0:struct
        let s_101_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call _get_HCR_EL2_Type_TGE(s_101_1)
        let s_101_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_101_1);
        // D s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // C s_101_4: const #1u : u8
        let s_101_4: bool = true;
        // C s_101_5: cast zx s_101_4 -> bv
        let s_101_5: Bits = Bits::new(s_101_4 as u128, 1u16);
        // D s_101_6: cmp-eq s_101_3 s_101_5
        let s_101_6: bool = ((s_101_3) == (s_101_5));
        // N s_101_7: branch s_101_6 b107 b102
        if s_101_6 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_102_0: read-var tea_bit:u8
        let s_102_0: bool = fn_state.tea_bit;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #1u : u8
        let s_102_2: bool = true;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#9792 <= s_102_4
        fn_state.gs_9792 = s_102_4;
        // N s_102_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_103_0: read-var gs#9792:u8
        let s_103_0: bool = fn_state.gs_9792;
        // N s_103_1: branch s_103_0 b106 b104
        if s_103_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_104_0: read-var fault:struct
        let s_104_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_104_1: call IsSecondStage(s_104_0)
        let s_104_1: bool = IsSecondStage(state, tracer, s_104_0);
        // D s_104_2: write-var gs#9793 <= s_104_1
        fn_state.gs_9793 = s_104_1;
        // N s_104_3: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_105_0: read-var gs#9793:u8
        let s_105_0: bool = fn_state.gs_9793;
        // D s_105_1: write-var gs#9794 <= s_105_0
        fn_state.gs_9794 = s_105_0;
        // N s_105_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_106_0: const #1u : u8
        let s_106_0: bool = true;
        // D s_106_1: write-var gs#9793 <= s_106_0
        fn_state.gs_9793 = s_106_0;
        // N s_106_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_107_0: const #1u : u8
        let s_107_0: bool = true;
        // D s_107_1: write-var gs#9792 <= s_107_0
        fn_state.gs_9792 = s_107_0;
        // N s_107_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_108_0: const #16975u : u32
        let s_108_0: u32 = 16975;
        // D s_108_1: read-reg s_108_0:u8
        let s_108_1: u8 = {
            let value = state.read_register::<u8>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // D s_108_2: cast zx s_108_1 -> bv
        let s_108_2: Bits = Bits::new(s_108_1 as u128, 2u16);
        // C s_108_3: const #448u : u32
        let s_108_3: u32 = 448;
        // D s_108_4: read-reg s_108_3:u8
        let s_108_4: u8 = {
            let value = state.read_register::<u8>(s_108_3 as isize);
            tracer.read_register(s_108_3 as isize, value);
            value
        };
        // D s_108_5: cast zx s_108_4 -> bv
        let s_108_5: Bits = Bits::new(s_108_4 as u128, 2u16);
        // D s_108_6: cmp-eq s_108_2 s_108_5
        let s_108_6: bool = ((s_108_2) == (s_108_5));
        // D s_108_7: write-var gs#9791 <= s_108_6
        fn_state.gs_9791 = s_108_6;
        // N s_108_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EL2Enabled(s_109_0)
        let s_109_1: bool = EL2Enabled(state, tracer, s_109_0);
        // D s_109_2: write-var gs#9790 <= s_109_1
        fn_state.gs_9790 = s_109_1;
        // N s_109_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_110_0: read-var tea_bit:u8
        let s_110_0: bool = fn_state.tea_bit;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #1u : u8
        let s_110_2: bool = true;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // N s_110_5: branch s_110_4 b116 b111
        if s_110_4 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_111_0: read-var fault.0:struct
        let s_111_0: ProductType9878976b5bcce9c9 = fn_state.fault._0;
        // D s_111_1: write-var ga#6994 <= s_111_0
        fn_state.ga_6994 = s_111_0;
        // D s_111_2: read-var ga#6994.1:struct
        let s_111_2: u32 = fn_state.ga_6994._1;
        // C s_111_3: const #9u : u32
        let s_111_3: u32 = 9;
        // D s_111_4: cmp-eq s_111_2 s_111_3
        let s_111_4: bool = ((s_111_2) == (s_111_3));
        // D s_111_5: write-var gs#9795 <= s_111_4
        fn_state.gs_9795 = s_111_4;
        // N s_111_6: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_112_0: read-var gs#9795:u8
        let s_112_0: bool = fn_state.gs_9795;
        // N s_112_1: branch s_112_0 b115 b113
        if s_112_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_113_0: read-var fault:struct
        let s_113_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_113_1: call IsSecondStage(s_113_0)
        let s_113_1: bool = IsSecondStage(state, tracer, s_113_0);
        // D s_113_2: write-var gs#9796 <= s_113_1
        fn_state.gs_9796 = s_113_1;
        // N s_113_3: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_114_0: read-var gs#9796:u8
        let s_114_0: bool = fn_state.gs_9796;
        // D s_114_1: write-var route_to_el2 <= s_114_0
        fn_state.route_to_el2 = s_114_0;
        // N s_114_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_115_0: const #1u : u8
        let s_115_0: bool = true;
        // D s_115_1: write-var gs#9796 <= s_115_0
        fn_state.gs_9796 = s_115_0;
        // N s_115_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_116_0: const #1u : u8
        let s_116_0: bool = true;
        // D s_116_1: write-var gs#9795 <= s_116_0
        fn_state.gs_9795 = s_116_0;
        // N s_116_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_117_0: const #16975u : u32
        let s_117_0: u32 = 16975;
        // D s_117_1: read-reg s_117_0:u8
        let s_117_1: u8 = {
            let value = state.read_register::<u8>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // D s_117_2: cast zx s_117_1 -> bv
        let s_117_2: Bits = Bits::new(s_117_1 as u128, 2u16);
        // C s_117_3: const #440u : u32
        let s_117_3: u32 = 440;
        // D s_117_4: read-reg s_117_3:u8
        let s_117_4: u8 = {
            let value = state.read_register::<u8>(s_117_3 as isize);
            tracer.read_register(s_117_3 as isize, value);
            value
        };
        // D s_117_5: cast zx s_117_4 -> bv
        let s_117_5: Bits = Bits::new(s_117_4 as u128, 2u16);
        // D s_117_6: cmp-eq s_117_2 s_117_5
        let s_117_6: bool = ((s_117_2) == (s_117_5));
        // D s_117_7: write-var gs#9788 <= s_117_6
        fn_state.gs_9788 = s_117_6;
        // N s_117_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EL2Enabled(s_118_0)
        let s_118_1: bool = EL2Enabled(state, tracer, s_118_0);
        // D s_118_2: write-var gs#9787 <= s_118_1
        fn_state.gs_9787 = s_118_1;
        // N s_118_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_119_0: const #102552u : u32
        let s_119_0: u32 = 102552;
        // D s_119_1: read-reg s_119_0:struct
        let s_119_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // D s_119_2: call _get_HCR_EL2_Type_TEA(s_119_1)
        let s_119_2: bool = u_get_HCR_EL2_Type_TEA(state, tracer, s_119_1);
        // D s_119_3: write-var tea_bit <= s_119_2
        fn_state.tea_bit = s_119_2;
        // N s_119_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_120_0: const #424u : u32
        let s_120_0: u32 = 424;
        // D s_120_1: read-reg s_120_0:u8
        let s_120_1: u8 = {
            let value = state.read_register::<u8>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // C s_120_2: const #2u : u8
        let s_120_2: u8 = 2;
        // D s_120_3: cmp-lt s_120_1 s_120_2
        let s_120_3: bool = ((s_120_1) < (s_120_2));
        // N s_120_4: branch s_120_3 b123 b121
        if s_120_3 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#9786 <= s_121_0
        fn_state.gs_9786 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_122_0: read-var gs#9786:u8
        let s_122_0: bool = fn_state.gs_9786;
        // D s_122_1: write-var route_to_el3 <= s_122_0
        fn_state.route_to_el3 = s_122_0;
        // N s_122_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call EffectiveEA(s_123_0)
        let s_123_1: bool = EffectiveEA(state, tracer, s_123_0);
        // S s_123_2: cast zx s_123_1 -> bv
        let s_123_2: Bits = Bits::new(s_123_1 as u128, 1u16);
        // C s_123_3: const #1u : u8
        let s_123_3: bool = true;
        // C s_123_4: cast zx s_123_3 -> bv
        let s_123_4: Bits = Bits::new(s_123_3 as u128, 1u16);
        // S s_123_5: cmp-eq s_123_2 s_123_4
        let s_123_5: bool = ((s_123_2) == (s_123_4));
        // D s_123_6: write-var gs#9786 <= s_123_5
        fn_state.gs_9786 = s_123_5;
        // N s_123_7: jump b122
        return block_122(state, tracer, fn_state);
    }
}
