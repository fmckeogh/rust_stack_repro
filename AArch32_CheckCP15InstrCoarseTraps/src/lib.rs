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
use EL2Enabled::*;
use u__id::*;
use HCR_read::*;
use ELUsingAArch32::*;
use AArch64_CheckCP15InstrCoarseTraps::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TIDCP::*;
use AArch32_SystemAccessTrap::*;
use common::*;
pub fn AArch32_CheckCP15InstrCoarseTraps<T: Tracer>(
    state: &mut State,
    tracer: &T,
    CRn: i128,
    nreg: i128,
    CRm: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_30844: bool,
        gs_30848: bool,
        gs_30860: bool,
        gs_30857: bool,
        gs_30877: bool,
        gs_30876: bool,
        gs_30847: bool,
        gs_30893: bool,
        gs_30846: bool,
        trapped_encoding: bool,
        gs_30833: bool,
        ga_23998: ProductType700c18a878c5601b,
        gs_30834: bool,
        gs_30875: bool,
        gs_30899: bool,
        gs_30849: bool,
        gs_30898: bool,
        gs_30859: bool,
        gs_30880: bool,
        gs_30890: bool,
        gs_30879: bool,
        gs_30872: bool,
        gs_30897: bool,
        gs_30881: bool,
        gs_30878: bool,
        gs_30895: bool,
        gs_30884: bool,
        gs_30873: bool,
        gs_30850: bool,
        gs_30882: bool,
        majorshadow_567: i128,
        gs_30901: bool,
        gs_30858: bool,
        gs_30832: bool,
        gs_30883: bool,
        gs_30845: bool,
        gs_30874: bool,
        ga_23991: i128,
        gs_30856: bool,
        CRn: i128,
        nreg: i128,
        CRm: i128,
    }
    let fn_state = FunctionState {
        CRn,
        nreg,
        CRm,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-eq s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) == (s_0_5));
        // N s_0_7: branch s_0_6 b113 b1
        if s_0_6 {
            return block_113(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#30834 <= s_1_0
        fn_state.gs_30834 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#30834:u8
        let s_2_0: bool = fn_state.gs_30834;
        // N s_2_1: branch s_2_0 b112 b3
        if s_2_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #9s : i
        let s_4_0: i128 = 9;
        // D s_4_1: read-var CRn:i
        let s_4_1: i128 = fn_state.CRn;
        // D s_4_2: cmp-eq s_4_1 s_4_0
        let s_4_2: bool = ((s_4_1) == (s_4_0));
        // N s_4_3: branch s_4_2 b93 b5
        if s_4_2 {
            return block_93(state, tracer, fn_state);
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
        // D s_5_1: write-var gs#30850 <= s_5_0
        fn_state.gs_30850 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#30850:u8
        let s_6_0: bool = fn_state.gs_30850;
        // N s_6_1: branch s_6_0 b92 b7
        if s_6_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #10s : i
        let s_7_0: i128 = 10;
        // D s_7_1: read-var CRn:i
        let s_7_1: i128 = fn_state.CRn;
        // D s_7_2: cmp-eq s_7_1 s_7_0
        let s_7_2: bool = ((s_7_1) == (s_7_0));
        // N s_7_3: branch s_7_2 b82 b8
        if s_7_2 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#30859 <= s_8_0
        fn_state.gs_30859 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#30859:u8
        let s_9_0: bool = fn_state.gs_30859;
        // D s_9_1: write-var gs#30860 <= s_9_0
        fn_state.gs_30860 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#30860:u8
        let s_10_0: bool = fn_state.gs_30860;
        // N s_10_1: branch s_10_0 b81 b11
        if s_10_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #11s : i
        let s_11_0: i128 = 11;
        // D s_11_1: read-var CRn:i
        let s_11_1: i128 = fn_state.CRn;
        // D s_11_2: cmp-eq s_11_1 s_11_0
        let s_11_2: bool = ((s_11_1) == (s_11_0));
        // N s_11_3: branch s_11_2 b53 b12
        if s_11_2 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#30881 <= s_12_0
        fn_state.gs_30881 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#30881:u8
        let s_13_0: bool = fn_state.gs_30881;
        // D s_13_1: write-var gs#30882 <= s_13_0
        fn_state.gs_30882 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#30882:u8
        let s_14_0: bool = fn_state.gs_30882;
        // D s_14_1: write-var trapped_encoding <= s_14_0
        fn_state.trapped_encoding = s_14_0;
        // C s_14_2: const #16975u : u32
        let s_14_2: u32 = 16975;
        // D s_14_3: read-reg s_14_2:u8
        let s_14_3: u8 = {
            let value = state.read_register::<u8>(s_14_2 as isize);
            tracer.read_register(s_14_2 as isize, value);
            value
        };
        // D s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 2u16);
        // C s_14_5: const #448u : u32
        let s_14_5: u32 = 448;
        // D s_14_6: read-reg s_14_5:u8
        let s_14_6: u8 = {
            let value = state.read_register::<u8>(s_14_5 as isize);
            tracer.read_register(s_14_5 as isize, value);
            value
        };
        // D s_14_7: cast zx s_14_6 -> bv
        let s_14_7: Bits = Bits::new(s_14_6 as u128, 2u16);
        // D s_14_8: cmp-eq s_14_4 s_14_7
        let s_14_8: bool = ((s_14_4) == (s_14_7));
        // N s_14_9: branch s_14_8 b52 b15
        if s_14_8 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #16975u : u32
        let s_15_0: u32 = 16975;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 2u16);
        // C s_15_3: const #440u : u32
        let s_15_3: u32 = 440;
        // D s_15_4: read-reg s_15_3:u8
        let s_15_4: u8 = {
            let value = state.read_register::<u8>(s_15_3 as isize);
            tracer.read_register(s_15_3 as isize, value);
            value
        };
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // D s_15_6: cmp-eq s_15_2 s_15_5
        let s_15_6: bool = ((s_15_2) == (s_15_5));
        // D s_15_7: write-var gs#30883 <= s_15_6
        fn_state.gs_30883 = s_15_6;
        // N s_15_8: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#30883:u8
        let s_16_0: bool = fn_state.gs_30883;
        // N s_16_1: branch s_16_0 b51 b17
        if s_16_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#30884 <= s_17_0
        fn_state.gs_30884 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#30884:u8
        let s_18_0: bool = fn_state.gs_30884;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // D s_20_1: read-var nreg:i
        let s_20_1: i128 = fn_state.nreg;
        // D s_20_2: cmp-eq s_20_1 s_20_0
        let s_20_2: bool = ((s_20_1) == (s_20_0));
        // N s_20_3: branch s_20_2 b50 b21
        if s_20_2 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var CRm:i
        let s_21_0: i128 = fn_state.CRm;
        // D s_21_1: write-var ga#23991 <= s_21_0
        fn_state.ga_23991 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var ga#23991:i
        let s_22_0: i128 = fn_state.ga_23991;
        // D s_22_1: write-var majorshadow#567 <= s_22_0
        fn_state.majorshadow_567 = s_22_0;
        // D s_22_2: read-var majorshadow#567:i
        let s_22_2: i128 = fn_state.majorshadow_567;
        // D s_22_3: call __id(s_22_2)
        let s_22_3: i128 = u__id(state, tracer, s_22_2);
        // C s_22_4: const #0s : i
        let s_22_4: i128 = 0;
        // D s_22_5: cmp-le s_22_4 s_22_3
        let s_22_5: bool = ((s_22_4) <= (s_22_3));
        // N s_22_6: branch s_22_5 b49 b23
        if s_22_5 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#30890 <= s_23_0
        fn_state.gs_30890 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#30890:u8
        let s_24_0: bool = fn_state.gs_30890;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // C s_24_2: const #4s : i
        let s_24_2: i128 = 4;
        // D s_24_3: read-var majorshadow#567:i
        let s_24_3: i128 = fn_state.majorshadow_567;
        // D s_24_4: cmp-eq s_24_3 s_24_2
        let s_24_4: bool = ((s_24_3) == (s_24_2));
        // N s_24_5: branch s_24_4 b48 b25
        if s_24_4 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #14s : i
        let s_25_0: i128 = 14;
        // D s_25_1: read-var majorshadow#567:i
        let s_25_1: i128 = fn_state.majorshadow_567;
        // D s_25_2: cmp-eq s_25_1 s_25_0
        let s_25_2: bool = ((s_25_1) == (s_25_0));
        // D s_25_3: write-var gs#30893 <= s_25_2
        fn_state.gs_30893 = s_25_2;
        // N s_25_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#30893:u8
        let s_26_0: bool = fn_state.gs_30893;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // N s_26_2: branch s_26_1 b47 b27
        if s_26_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#30895 <= s_27_0
        fn_state.gs_30895 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#30895:u8
        let s_28_0: bool = fn_state.gs_30895;
        // N s_28_1: branch s_28_0 b46 b29
        if s_28_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call HCR_read(s_29_0)
        let s_29_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_29_0);
        // S s_29_2: call _get_HCR_Type_TIDCP(s_29_1)
        let s_29_2: bool = u_get_HCR_Type_TIDCP(state, tracer, s_29_1);
        // S s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // C s_29_4: const #1u : u8
        let s_29_4: bool = true;
        // C s_29_5: cast zx s_29_4 -> bv
        let s_29_5: Bits = Bits::new(s_29_4 as u128, 1u16);
        // S s_29_6: cmp-eq s_29_3 s_29_5
        let s_29_6: bool = ((s_29_3) == (s_29_5));
        // N s_29_7: branch s_29_6 b45 b30
        if s_29_6 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#30897 <= s_30_0
        fn_state.gs_30897 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#30897:u8
        let s_31_0: bool = fn_state.gs_30897;
        // N s_31_1: branch s_31_0 b44 b32
        if s_31_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#30898 <= s_32_0
        fn_state.gs_30898 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#30898:u8
        let s_33_0: bool = fn_state.gs_30898;
        // D s_33_1: write-var gs#30899 <= s_33_0
        fn_state.gs_30899 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#30899:u8
        let s_34_0: bool = fn_state.gs_30899;
        // N s_34_1: branch s_34_0 b36 b35
        if s_34_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #16975u : u32
        let s_36_0: u32 = 16975;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: cast zx s_36_1 -> bv
        let s_36_2: Bits = Bits::new(s_36_1 as u128, 2u16);
        // C s_36_3: const #448u : u32
        let s_36_3: u32 = 448;
        // D s_36_4: read-reg s_36_3:u8
        let s_36_4: u8 = {
            let value = state.read_register::<u8>(s_36_3 as isize);
            tracer.read_register(s_36_3 as isize, value);
            value
        };
        // D s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 2u16);
        // D s_36_6: cmp-eq s_36_2 s_36_5
        let s_36_6: bool = ((s_36_2) == (s_36_5));
        // N s_36_7: branch s_36_6 b43 b37
        if s_36_6 {
            return block_43(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#30901 <= s_37_0
        fn_state.gs_30901 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#30901:u8
        let s_38_0: bool = fn_state.gs_30901;
        // N s_38_1: branch s_38_0 b42 b39
        if s_38_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #432u : u32
        let s_39_0: u32 = 432;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call ELUsingAArch32(s_39_1)
        let s_39_2: bool = ELUsingAArch32(state, tracer, s_39_1);
        // N s_39_3: branch s_39_2 b41 b40
        if s_39_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #3u : u8
        let s_40_0: u8 = 3;
        // C s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 4u16);
        // C s_40_2: cast zx s_40_1 -> i
        let s_40_2: i128 = (s_40_1.value() as i128);
        // C s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // C s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // C s_40_5: const #432u : u32
        let s_40_5: u32 = 432;
        // D s_40_6: read-reg s_40_5:u8
        let s_40_6: u8 = {
            let value = state.read_register::<u8>(s_40_5 as isize);
            tracer.read_register(s_40_5 as isize, value);
            value
        };
        // D s_40_7: call AArch64_AArch32SystemAccessTrap(s_40_6, s_40_4)
        let s_40_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_40_6, s_40_4);
        // N s_40_8: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #3u : u8
        let s_41_0: u8 = 3;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 4u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #400u : u32
        let s_41_5: u32 = 400;
        // D s_41_6: read-reg s_41_5:u8
        let s_41_6: u8 = {
            let value = state.read_register::<u8>(s_41_5 as isize);
            tracer.read_register(s_41_5 as isize, value);
            value
        };
        // D s_41_7: call AArch32_SystemAccessTrap(s_41_6, s_41_4)
        let s_41_7: () = AArch32_SystemAccessTrap(state, tracer, s_41_6, s_41_4);
        // N s_41_8: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: panic
        panic!("{:?}", ());
        // N s_42_1: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #"UNDEF unallocated CP15 access at EL0" : str
        let s_43_0: &'static str = "UNDEF unallocated CP15 access at EL0";
        // S s_43_1: call __IMPDEF_boolean(s_43_0)
        let s_43_1: bool = u__IMPDEF_boolean(state, tracer, s_43_0);
        // D s_43_2: write-var gs#30901 <= s_43_1
        fn_state.gs_30901 = s_43_1;
        // N s_43_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var trapped_encoding:u8
        let s_44_0: bool = fn_state.trapped_encoding;
        // D s_44_1: write-var gs#30898 <= s_44_0
        fn_state.gs_30898 = s_44_0;
        // N s_44_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1s : i
        let s_45_0: i128 = 1;
        // D s_45_1: read-var nreg:i
        let s_45_1: i128 = fn_state.nreg;
        // D s_45_2: cmp-eq s_45_1 s_45_0
        let s_45_2: bool = ((s_45_1) == (s_45_0));
        // D s_45_3: write-var gs#30897 <= s_45_2
        fn_state.gs_30897 = s_45_2;
        // N s_45_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #1u : u8
        let s_46_0: bool = true;
        // D s_46_1: write-var gs#30899 <= s_46_0
        fn_state.gs_30899 = s_46_0;
        // N s_46_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call HSTR_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_47_0);
        // D s_47_2: write-var ga#23998 <= s_47_1
        fn_state.ga_23998 = s_47_1;
        // D s_47_3: read-var ga#23998.0:struct
        let s_47_3: u32 = fn_state.ga_23998._0;
        // D s_47_4: cast zx s_47_3 -> bv
        let s_47_4: Bits = Bits::new(s_47_3 as u128, 32u16);
        // D s_47_5: read-var majorshadow#567:i
        let s_47_5: i128 = fn_state.majorshadow_567;
        // C s_47_6: const #1u : u64
        let s_47_6: u64 = 1;
        // D s_47_7: bit-extract s_47_4 s_47_5 s_47_6
        let s_47_7: Bits = (Bits::new(
            ((s_47_4) >> (s_47_5)).value(),
            u16::try_from(s_47_6).unwrap(),
        ));
        // D s_47_8: cast reint s_47_7 -> u8
        let s_47_8: bool = ((s_47_7.value()) != 0);
        // C s_47_9: const #0s : i
        let s_47_9: i128 = 0;
        // C s_47_10: const #0u : u64
        let s_47_10: u64 = 0;
        // D s_47_11: cast zx s_47_8 -> u64
        let s_47_11: u64 = (s_47_8 as u64);
        // C s_47_12: const #1u : u64
        let s_47_12: u64 = 1;
        // D s_47_13: and s_47_11 s_47_12
        let s_47_13: u64 = ((s_47_11) & (s_47_12));
        // D s_47_14: cmp-eq s_47_13 s_47_12
        let s_47_14: bool = ((s_47_13) == (s_47_12));
        // D s_47_15: lsl s_47_11 s_47_9
        let s_47_15: u64 = s_47_11 << s_47_9;
        // D s_47_16: or s_47_10 s_47_15
        let s_47_16: u64 = ((s_47_10) | (s_47_15));
        // D s_47_17: cmpl s_47_15
        let s_47_17: u64 = !s_47_15;
        // D s_47_18: and s_47_10 s_47_17
        let s_47_18: u64 = ((s_47_10) & (s_47_17));
        // D s_47_19: select s_47_14 s_47_16 s_47_18
        let s_47_19: u64 = if s_47_14 { s_47_16 } else { s_47_18 };
        // D s_47_20: cast trunc s_47_19 -> u8
        let s_47_20: bool = ((s_47_19) != 0);
        // D s_47_21: cast zx s_47_20 -> bv
        let s_47_21: Bits = Bits::new(s_47_20 as u128, 1u16);
        // C s_47_22: const #1u : u8
        let s_47_22: bool = true;
        // C s_47_23: cast zx s_47_22 -> bv
        let s_47_23: Bits = Bits::new(s_47_22 as u128, 1u16);
        // D s_47_24: cmp-eq s_47_21 s_47_23
        let s_47_24: bool = ((s_47_21) == (s_47_23));
        // D s_47_25: write-var gs#30895 <= s_47_24
        fn_state.gs_30895 = s_47_24;
        // N s_47_26: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: write-var gs#30893 <= s_48_0
        fn_state.gs_30893 = s_48_0;
        // N s_48_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var majorshadow#567:i
        let s_49_0: i128 = fn_state.majorshadow_567;
        // D s_49_1: call __id(s_49_0)
        let s_49_1: i128 = u__id(state, tracer, s_49_0);
        // C s_49_2: const #32s : i
        let s_49_2: i128 = 32;
        // D s_49_3: cmp-lt s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) < (s_49_2));
        // D s_49_4: write-var gs#30890 <= s_49_3
        fn_state.gs_30890 = s_49_3;
        // N s_49_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var CRn:i
        let s_50_0: i128 = fn_state.CRn;
        // D s_50_1: write-var ga#23991 <= s_50_0
        fn_state.ga_23991 = s_50_0;
        // N s_50_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call EL2Enabled(s_51_0)
        let s_51_1: bool = EL2Enabled(state, tracer, s_51_0);
        // D s_51_2: write-var gs#30884 <= s_51_1
        fn_state.gs_30884 = s_51_1;
        // N s_51_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#30883 <= s_52_0
        fn_state.gs_30883 = s_52_0;
        // N s_52_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0s : i
        let s_53_0: i128 = 0;
        // D s_53_1: read-var CRm:i
        let s_53_1: i128 = fn_state.CRm;
        // D s_53_2: cmp-eq s_53_1 s_53_0
        let s_53_2: bool = ((s_53_1) == (s_53_0));
        // N s_53_3: branch s_53_2 b80 b54
        if s_53_2 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1s : i
        let s_54_0: i128 = 1;
        // D s_54_1: read-var CRm:i
        let s_54_1: i128 = fn_state.CRm;
        // D s_54_2: cmp-eq s_54_1 s_54_0
        let s_54_2: bool = ((s_54_1) == (s_54_0));
        // N s_54_3: branch s_54_2 b79 b55
        if s_54_2 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #2s : i
        let s_55_0: i128 = 2;
        // D s_55_1: read-var CRm:i
        let s_55_1: i128 = fn_state.CRm;
        // D s_55_2: cmp-eq s_55_1 s_55_0
        let s_55_2: bool = ((s_55_1) == (s_55_0));
        // N s_55_3: branch s_55_2 b78 b56
        if s_55_2 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #3s : i
        let s_56_0: i128 = 3;
        // D s_56_1: read-var CRm:i
        let s_56_1: i128 = fn_state.CRm;
        // D s_56_2: cmp-eq s_56_1 s_56_0
        let s_56_2: bool = ((s_56_1) == (s_56_0));
        // N s_56_3: branch s_56_2 b77 b57
        if s_56_2 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #4s : i
        let s_57_0: i128 = 4;
        // D s_57_1: read-var CRm:i
        let s_57_1: i128 = fn_state.CRm;
        // D s_57_2: cmp-eq s_57_1 s_57_0
        let s_57_2: bool = ((s_57_1) == (s_57_0));
        // N s_57_3: branch s_57_2 b76 b58
        if s_57_2 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #5s : i
        let s_58_0: i128 = 5;
        // D s_58_1: read-var CRm:i
        let s_58_1: i128 = fn_state.CRm;
        // D s_58_2: cmp-eq s_58_1 s_58_0
        let s_58_2: bool = ((s_58_1) == (s_58_0));
        // N s_58_3: branch s_58_2 b75 b59
        if s_58_2 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #6s : i
        let s_59_0: i128 = 6;
        // D s_59_1: read-var CRm:i
        let s_59_1: i128 = fn_state.CRm;
        // D s_59_2: cmp-eq s_59_1 s_59_0
        let s_59_2: bool = ((s_59_1) == (s_59_0));
        // N s_59_3: branch s_59_2 b74 b60
        if s_59_2 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #7s : i
        let s_60_0: i128 = 7;
        // D s_60_1: read-var CRm:i
        let s_60_1: i128 = fn_state.CRm;
        // D s_60_2: cmp-eq s_60_1 s_60_0
        let s_60_2: bool = ((s_60_1) == (s_60_0));
        // N s_60_3: branch s_60_2 b73 b61
        if s_60_2 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #8s : i
        let s_61_0: i128 = 8;
        // D s_61_1: read-var CRm:i
        let s_61_1: i128 = fn_state.CRm;
        // D s_61_2: cmp-eq s_61_1 s_61_0
        let s_61_2: bool = ((s_61_1) == (s_61_0));
        // N s_61_3: branch s_61_2 b72 b62
        if s_61_2 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #15s : i
        let s_62_0: i128 = 15;
        // D s_62_1: read-var CRm:i
        let s_62_1: i128 = fn_state.CRm;
        // D s_62_2: cmp-eq s_62_1 s_62_0
        let s_62_2: bool = ((s_62_1) == (s_62_0));
        // D s_62_3: write-var gs#30872 <= s_62_2
        fn_state.gs_30872 = s_62_2;
        // N s_62_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#30872:u8
        let s_63_0: bool = fn_state.gs_30872;
        // D s_63_1: write-var gs#30873 <= s_63_0
        fn_state.gs_30873 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#30873:u8
        let s_64_0: bool = fn_state.gs_30873;
        // D s_64_1: write-var gs#30874 <= s_64_0
        fn_state.gs_30874 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#30874:u8
        let s_65_0: bool = fn_state.gs_30874;
        // D s_65_1: write-var gs#30875 <= s_65_0
        fn_state.gs_30875 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#30875:u8
        let s_66_0: bool = fn_state.gs_30875;
        // D s_66_1: write-var gs#30876 <= s_66_0
        fn_state.gs_30876 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#30876:u8
        let s_67_0: bool = fn_state.gs_30876;
        // D s_67_1: write-var gs#30877 <= s_67_0
        fn_state.gs_30877 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#30877:u8
        let s_68_0: bool = fn_state.gs_30877;
        // D s_68_1: write-var gs#30878 <= s_68_0
        fn_state.gs_30878 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#30878:u8
        let s_69_0: bool = fn_state.gs_30878;
        // D s_69_1: write-var gs#30879 <= s_69_0
        fn_state.gs_30879 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#30879:u8
        let s_70_0: bool = fn_state.gs_30879;
        // D s_70_1: write-var gs#30880 <= s_70_0
        fn_state.gs_30880 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#30880:u8
        let s_71_0: bool = fn_state.gs_30880;
        // D s_71_1: write-var gs#30881 <= s_71_0
        fn_state.gs_30881 = s_71_0;
        // N s_71_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#30872 <= s_72_0
        fn_state.gs_30872 = s_72_0;
        // N s_72_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #1u : u8
        let s_73_0: bool = true;
        // D s_73_1: write-var gs#30873 <= s_73_0
        fn_state.gs_30873 = s_73_0;
        // N s_73_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: write-var gs#30874 <= s_74_0
        fn_state.gs_30874 = s_74_0;
        // N s_74_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#30875 <= s_75_0
        fn_state.gs_30875 = s_75_0;
        // N s_75_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#30876 <= s_76_0
        fn_state.gs_30876 = s_76_0;
        // N s_76_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #1u : u8
        let s_77_0: bool = true;
        // D s_77_1: write-var gs#30877 <= s_77_0
        fn_state.gs_30877 = s_77_0;
        // N s_77_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #1u : u8
        let s_78_0: bool = true;
        // D s_78_1: write-var gs#30878 <= s_78_0
        fn_state.gs_30878 = s_78_0;
        // N s_78_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var gs#30879 <= s_79_0
        fn_state.gs_30879 = s_79_0;
        // N s_79_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#30880 <= s_80_0
        fn_state.gs_30880 = s_80_0;
        // N s_80_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #1u : u8
        let s_81_0: bool = true;
        // D s_81_1: write-var gs#30882 <= s_81_0
        fn_state.gs_30882 = s_81_0;
        // N s_81_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0s : i
        let s_82_0: i128 = 0;
        // D s_82_1: read-var CRm:i
        let s_82_1: i128 = fn_state.CRm;
        // D s_82_2: cmp-eq s_82_1 s_82_0
        let s_82_2: bool = ((s_82_1) == (s_82_0));
        // N s_82_3: branch s_82_2 b91 b83
        if s_82_2 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #1s : i
        let s_83_0: i128 = 1;
        // D s_83_1: read-var CRm:i
        let s_83_1: i128 = fn_state.CRm;
        // D s_83_2: cmp-eq s_83_1 s_83_0
        let s_83_2: bool = ((s_83_1) == (s_83_0));
        // N s_83_3: branch s_83_2 b90 b84
        if s_83_2 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #4s : i
        let s_84_0: i128 = 4;
        // D s_84_1: read-var CRm:i
        let s_84_1: i128 = fn_state.CRm;
        // D s_84_2: cmp-eq s_84_1 s_84_0
        let s_84_2: bool = ((s_84_1) == (s_84_0));
        // N s_84_3: branch s_84_2 b89 b85
        if s_84_2 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #8s : i
        let s_85_0: i128 = 8;
        // D s_85_1: read-var CRm:i
        let s_85_1: i128 = fn_state.CRm;
        // D s_85_2: cmp-eq s_85_1 s_85_0
        let s_85_2: bool = ((s_85_1) == (s_85_0));
        // D s_85_3: write-var gs#30856 <= s_85_2
        fn_state.gs_30856 = s_85_2;
        // N s_85_4: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#30856:u8
        let s_86_0: bool = fn_state.gs_30856;
        // D s_86_1: write-var gs#30857 <= s_86_0
        fn_state.gs_30857 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#30857:u8
        let s_87_0: bool = fn_state.gs_30857;
        // D s_87_1: write-var gs#30858 <= s_87_0
        fn_state.gs_30858 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#30858:u8
        let s_88_0: bool = fn_state.gs_30858;
        // D s_88_1: write-var gs#30859 <= s_88_0
        fn_state.gs_30859 = s_88_0;
        // N s_88_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #1u : u8
        let s_89_0: bool = true;
        // D s_89_1: write-var gs#30856 <= s_89_0
        fn_state.gs_30856 = s_89_0;
        // N s_89_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #1u : u8
        let s_90_0: bool = true;
        // D s_90_1: write-var gs#30857 <= s_90_0
        fn_state.gs_30857 = s_90_0;
        // N s_90_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #1u : u8
        let s_91_0: bool = true;
        // D s_91_1: write-var gs#30858 <= s_91_0
        fn_state.gs_30858 = s_91_0;
        // N s_91_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #1u : u8
        let s_92_0: bool = true;
        // D s_92_1: write-var gs#30860 <= s_92_0
        fn_state.gs_30860 = s_92_0;
        // N s_92_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0s : i
        let s_93_0: i128 = 0;
        // D s_93_1: read-var CRm:i
        let s_93_1: i128 = fn_state.CRm;
        // D s_93_2: cmp-eq s_93_1 s_93_0
        let s_93_2: bool = ((s_93_1) == (s_93_0));
        // N s_93_3: branch s_93_2 b111 b94
        if s_93_2 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #1s : i
        let s_94_0: i128 = 1;
        // D s_94_1: read-var CRm:i
        let s_94_1: i128 = fn_state.CRm;
        // D s_94_2: cmp-eq s_94_1 s_94_0
        let s_94_2: bool = ((s_94_1) == (s_94_0));
        // N s_94_3: branch s_94_2 b110 b95
        if s_94_2 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #2s : i
        let s_95_0: i128 = 2;
        // D s_95_1: read-var CRm:i
        let s_95_1: i128 = fn_state.CRm;
        // D s_95_2: cmp-eq s_95_1 s_95_0
        let s_95_2: bool = ((s_95_1) == (s_95_0));
        // N s_95_3: branch s_95_2 b109 b96
        if s_95_2 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #5s : i
        let s_96_0: i128 = 5;
        // D s_96_1: read-var CRm:i
        let s_96_1: i128 = fn_state.CRm;
        // D s_96_2: cmp-eq s_96_1 s_96_0
        let s_96_2: bool = ((s_96_1) == (s_96_0));
        // N s_96_3: branch s_96_2 b108 b97
        if s_96_2 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #6s : i
        let s_97_0: i128 = 6;
        // D s_97_1: read-var CRm:i
        let s_97_1: i128 = fn_state.CRm;
        // D s_97_2: cmp-eq s_97_1 s_97_0
        let s_97_2: bool = ((s_97_1) == (s_97_0));
        // N s_97_3: branch s_97_2 b107 b98
        if s_97_2 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #7s : i
        let s_98_0: i128 = 7;
        // D s_98_1: read-var CRm:i
        let s_98_1: i128 = fn_state.CRm;
        // D s_98_2: cmp-eq s_98_1 s_98_0
        let s_98_2: bool = ((s_98_1) == (s_98_0));
        // N s_98_3: branch s_98_2 b106 b99
        if s_98_2 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #8s : i
        let s_99_0: i128 = 8;
        // D s_99_1: read-var CRm:i
        let s_99_1: i128 = fn_state.CRm;
        // D s_99_2: cmp-eq s_99_1 s_99_0
        let s_99_2: bool = ((s_99_1) == (s_99_0));
        // D s_99_3: write-var gs#30844 <= s_99_2
        fn_state.gs_30844 = s_99_2;
        // N s_99_4: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#30844:u8
        let s_100_0: bool = fn_state.gs_30844;
        // D s_100_1: write-var gs#30845 <= s_100_0
        fn_state.gs_30845 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#30845:u8
        let s_101_0: bool = fn_state.gs_30845;
        // D s_101_1: write-var gs#30846 <= s_101_0
        fn_state.gs_30846 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#30846:u8
        let s_102_0: bool = fn_state.gs_30846;
        // D s_102_1: write-var gs#30847 <= s_102_0
        fn_state.gs_30847 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#30847:u8
        let s_103_0: bool = fn_state.gs_30847;
        // D s_103_1: write-var gs#30848 <= s_103_0
        fn_state.gs_30848 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#30848:u8
        let s_104_0: bool = fn_state.gs_30848;
        // D s_104_1: write-var gs#30849 <= s_104_0
        fn_state.gs_30849 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#30849:u8
        let s_105_0: bool = fn_state.gs_30849;
        // D s_105_1: write-var gs#30850 <= s_105_0
        fn_state.gs_30850 = s_105_0;
        // N s_105_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #1u : u8
        let s_106_0: bool = true;
        // D s_106_1: write-var gs#30844 <= s_106_0
        fn_state.gs_30844 = s_106_0;
        // N s_106_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #1u : u8
        let s_107_0: bool = true;
        // D s_107_1: write-var gs#30845 <= s_107_0
        fn_state.gs_30845 = s_107_0;
        // N s_107_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #1u : u8
        let s_108_0: bool = true;
        // D s_108_1: write-var gs#30846 <= s_108_0
        fn_state.gs_30846 = s_108_0;
        // N s_108_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #1u : u8
        let s_109_0: bool = true;
        // D s_109_1: write-var gs#30847 <= s_109_0
        fn_state.gs_30847 = s_109_0;
        // N s_109_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #1u : u8
        let s_110_0: bool = true;
        // D s_110_1: write-var gs#30848 <= s_110_0
        fn_state.gs_30848 = s_110_0;
        // N s_110_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #1u : u8
        let s_111_0: bool = true;
        // D s_111_1: write-var gs#30849 <= s_111_0
        fn_state.gs_30849 = s_111_0;
        // N s_111_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var CRn:i
        let s_112_0: i128 = fn_state.CRn;
        // D s_112_1: read-var nreg:i
        let s_112_1: i128 = fn_state.nreg;
        // D s_112_2: read-var CRm:i
        let s_112_2: i128 = fn_state.CRm;
        // D s_112_3: call AArch64_CheckCP15InstrCoarseTraps(s_112_0, s_112_1, s_112_2)
        let s_112_3: () = AArch64_CheckCP15InstrCoarseTraps(
            state,
            tracer,
            s_112_0,
            s_112_1,
            s_112_2,
        );
        // N s_112_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #440u : u32
        let s_113_0: u32 = 440;
        // D s_113_1: read-reg s_113_0:u8
        let s_113_1: u8 = {
            let value = state.read_register::<u8>(s_113_0 as isize);
            tracer.read_register(s_113_0 as isize, value);
            value
        };
        // D s_113_2: call ELUsingAArch32(s_113_1)
        let s_113_2: bool = ELUsingAArch32(state, tracer, s_113_1);
        // D s_113_3: not s_113_2
        let s_113_3: bool = !s_113_2;
        // N s_113_4: branch s_113_3 b119 b114
        if s_113_3 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #() : ()
        let s_114_0: () = ();
        // S s_114_1: call EL2Enabled(s_114_0)
        let s_114_1: bool = EL2Enabled(state, tracer, s_114_0);
        // N s_114_2: branch s_114_1 b118 b115
        if s_114_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#30832 <= s_115_0
        fn_state.gs_30832 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#30832:u8
        let s_116_0: bool = fn_state.gs_30832;
        // D s_116_1: write-var gs#30833 <= s_116_0
        fn_state.gs_30833 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#30833:u8
        let s_117_0: bool = fn_state.gs_30833;
        // D s_117_1: write-var gs#30834 <= s_117_0
        fn_state.gs_30834 = s_117_0;
        // N s_117_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #432u : u32
        let s_118_0: u32 = 432;
        // D s_118_1: read-reg s_118_0:u8
        let s_118_1: u8 = {
            let value = state.read_register::<u8>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // D s_118_2: call ELUsingAArch32(s_118_1)
        let s_118_2: bool = ELUsingAArch32(state, tracer, s_118_1);
        // D s_118_3: not s_118_2
        let s_118_3: bool = !s_118_2;
        // D s_118_4: write-var gs#30832 <= s_118_3
        fn_state.gs_30832 = s_118_3;
        // N s_118_5: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #1u : u8
        let s_119_0: bool = true;
        // D s_119_1: write-var gs#30833 <= s_119_0
        fn_state.gs_30833 = s_119_0;
        // N s_119_2: jump b117
        return block_117(state, tracer, fn_state);
    }
}
