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
use u_get_MDCR_EL3_Type_E3BREW::*;
use PMUEvent::*;
use BRBEMispredictAllowed::*;
use u_get_BRBCR_EL1_Type_ERTN::*;
use Zeros::*;
use u_get_MDCR_EL3_Type_E3BREC::*;
use HaveBRBEv1p1::*;
use HaveTME::*;
use BranchEncCycleCount::*;
use u_get_BRBCR_EL2_Type_ERTN::*;
use PC_read::*;
use BranchMispredict::*;
use u_get_BRBFCR_EL1_Type_LASTFAILED::*;
use BranchRecordAllowed::*;
use UpdateBranchRecordBuffer::*;
use common::*;
pub fn BRBEExceptionReturn<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target_address_in: u64,
    source_el: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_26319: bool,
        gs_26317: bool,
        target_valid: bool,
        gs_26310: bool,
        ga_20527: ProductType1a93b8c16f53fb84,
        lastfailed: bool,
        tv: bool,
        gs_26301: bool,
        gs_26321: bool,
        cc: u16,
        gs_26320: bool,
        target_address: u64,
        source_valid: bool,
        transactional: bool,
        mispredict: bool,
        source_address: u64,
        ccu: bool,
        el: u8,
        branch_valid: bool,
        sv: bool,
        target_address_in: u64,
        source_el: u8,
    }
    let fn_state = FunctionState {
        target_address_in,
        source_el,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var target_address_in:u64
        let s_0_0: u64 = fn_state.target_address_in;
        // D s_0_1: write-var target_address <= s_0_0
        fn_state.target_address = s_0_0;
        // D s_0_2: read-var source_el:u8
        let s_0_2: u8 = fn_state.source_el;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 2u16);
        // C s_0_4: const #424u : u32
        let s_0_4: u32 = 424;
        // D s_0_5: read-reg s_0_4:u8
        let s_0_5: u8 = {
            let value = state.read_register::<u8>(s_0_4 as isize);
            tracer.read_register(s_0_4 as isize, value);
            value
        };
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 2u16);
        // D s_0_7: cmp-eq s_0_3 s_0_6
        let s_0_7: bool = ((s_0_3) == (s_0_6));
        // D s_0_8: not s_0_7
        let s_0_8: bool = !s_0_7;
        // N s_0_9: branch s_0_8 b50 b1
        if s_0_8 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveBRBEv1p1(s_1_0)
        let s_1_1: bool = HaveBRBEv1p1(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // N s_1_3: branch s_1_2 b49 b2
        if s_1_2 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #22712u : u32
        let s_2_0: u32 = 22712;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_MDCR_EL3_Type_E3BREC(s_2_1)
        let s_2_2: bool = u_get_MDCR_EL3_Type_E3BREC(state, tracer, s_2_1);
        // C s_2_3: const #22712u : u32
        let s_2_3: u32 = 22712;
        // D s_2_4: read-reg s_2_3:struct
        let s_2_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: call _get_MDCR_EL3_Type_E3BREW(s_2_4)
        let s_2_5: bool = u_get_MDCR_EL3_Type_E3BREW(state, tracer, s_2_4);
        // D s_2_6: cast zx s_2_2 -> bv
        let s_2_6: Bits = Bits::new(s_2_2 as u128, 1u16);
        // D s_2_7: cast zx s_2_5 -> bv
        let s_2_7: Bits = Bits::new(s_2_5 as u128, 1u16);
        // D s_2_8: cmp-eq s_2_6 s_2_7
        let s_2_8: bool = ((s_2_6) == (s_2_7));
        // D s_2_9: write-var gs#26301 <= s_2_8
        fn_state.gs_26301 = s_2_8;
        // N s_2_10: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#26301:u8
        let s_3_0: bool = fn_state.gs_26301;
        // N s_3_1: branch s_3_0 b48 b4
        if s_3_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var source_el:u8
        let s_5_0: u8 = fn_state.source_el;
        // D s_5_1: call BranchRecordAllowed(s_5_0)
        let s_5_1: bool = BranchRecordAllowed(state, tracer, s_5_0);
        // D s_5_2: write-var source_valid <= s_5_1
        fn_state.source_valid = s_5_1;
        // C s_5_3: const #16975u : u32
        let s_5_3: u32 = 16975;
        // D s_5_4: read-reg s_5_3:u8
        let s_5_4: u8 = {
            let value = state.read_register::<u8>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: call BranchRecordAllowed(s_5_4)
        let s_5_5: bool = BranchRecordAllowed(state, tracer, s_5_4);
        // D s_5_6: write-var target_valid <= s_5_5
        fn_state.target_valid = s_5_5;
        // D s_5_7: read-var source_valid:u8
        let s_5_7: bool = fn_state.source_valid;
        // N s_5_8: branch s_5_7 b47 b6
        if s_5_7 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var target_valid:u8
        let s_6_0: bool = fn_state.target_valid;
        // D s_6_1: write-var gs#26310 <= s_6_0
        fn_state.gs_26310 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#26310:u8
        let s_7_0: bool = fn_state.gs_26310;
        // D s_7_1: write-var branch_valid <= s_7_0
        fn_state.branch_valid = s_7_0;
        // D s_7_2: read-var branch_valid:u8
        let s_7_2: bool = fn_state.branch_valid;
        // N s_7_3: branch s_7_2 b10 b8
        if s_7_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var branch_valid:u8
        let s_9_0: bool = fn_state.branch_valid;
        // C s_9_1: const #14032u : u32
        let s_9_1: u32 = 14032;
        // N s_9_2: write-reg s_9_1 <= s_9_0
        let s_9_2: () = {
            state.write_register::<bool>(s_9_1 as isize, s_9_0);
            tracer.write_register(s_9_1 as isize, s_9_0);
        };
        // N s_9_3: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call BranchEncCycleCount(s_10_0)
        let s_10_1: ProductType1a93b8c16f53fb84 = BranchEncCycleCount(
            state,
            tracer,
            s_10_0,
        );
        // D s_10_2: write-var ga#20527 <= s_10_1
        fn_state.ga_20527 = s_10_1;
        // D s_10_3: read-var ga#20527.0:struct
        let s_10_3: bool = fn_state.ga_20527._0;
        // D s_10_4: read-var ga#20527.1:struct
        let s_10_4: u16 = fn_state.ga_20527._1;
        // D s_10_5: write-var ccu <= s_10_3
        fn_state.ccu = s_10_3;
        // D s_10_6: write-var cc <= s_10_4
        fn_state.cc = s_10_4;
        // C s_10_7: const #() : ()
        let s_10_7: () = ();
        // S s_10_8: call HaveTME(s_10_7)
        let s_10_8: bool = HaveTME(state, tracer, s_10_7);
        // N s_10_9: branch s_10_8 b46 b11
        if s_10_8 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var lastfailed <= s_11_0
        fn_state.lastfailed = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var source_valid:u8
        let s_12_0: bool = fn_state.source_valid;
        // N s_12_1: branch s_12_0 b45 b13
        if s_12_0 {
            return block_45(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#26317 <= s_13_0
        fn_state.gs_26317 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#26317:u8
        let s_14_0: bool = fn_state.gs_26317;
        // N s_14_1: branch s_14_0 b44 b15
        if s_14_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#26319 <= s_15_0
        fn_state.gs_26319 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#26319:u8
        let s_16_0: bool = fn_state.gs_26319;
        // N s_16_1: branch s_16_0 b43 b17
        if s_16_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_17_1: write-var transactional <= s_17_0
        fn_state.transactional = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var target_valid:u8
        let s_18_0: bool = fn_state.target_valid;
        // N s_18_1: branch s_18_0 b42 b19
        if s_18_0 {
            return block_42(state, tracer, fn_state);
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
        let s_19_0: u8 = 0;
        // D s_19_1: write-var el <= s_19_0
        fn_state.el = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var source_valid:u8
        let s_20_0: bool = fn_state.source_valid;
        // N s_20_1: branch s_20_0 b41 b21
        if s_20_0 {
            return block_41(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#26320 <= s_21_0
        fn_state.gs_26320 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#26320:u8
        let s_22_0: bool = fn_state.gs_26320;
        // N s_22_1: branch s_22_0 b40 b23
        if s_22_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#26321 <= s_23_0
        fn_state.gs_26321 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#26321:u8
        let s_24_0: bool = fn_state.gs_26321;
        // N s_24_1: branch s_24_0 b39 b25
        if s_24_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_25_1: write-var mispredict <= s_25_0
        fn_state.mispredict = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var source_valid:u8
        let s_26_0: bool = fn_state.source_valid;
        // N s_26_1: branch s_26_0 b38 b27
        if s_26_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_27_1: write-var sv <= s_27_0
        fn_state.sv = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var target_valid:u8
        let s_28_0: bool = fn_state.target_valid;
        // N s_28_1: branch s_28_0 b37 b29
        if s_28_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_29_1: write-var tv <= s_29_0
        fn_state.tv = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var source_valid:u8
        let s_30_0: bool = fn_state.source_valid;
        // N s_30_1: branch s_30_0 b36 b31
        if s_30_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #64s : i
        let s_31_0: i128 = 64;
        // S s_31_1: call Zeros(s_31_0)
        let s_31_1: Bits = Zeros(state, tracer, s_31_0);
        // S s_31_2: cast reint s_31_1 -> u64
        let s_31_2: u64 = (s_31_1.value() as u64);
        // D s_31_3: write-var source_address <= s_31_2
        fn_state.source_address = s_31_2;
        // N s_31_4: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var target_valid:u8
        let s_32_0: bool = fn_state.target_valid;
        // D s_32_1: not s_32_0
        let s_32_1: bool = !s_32_0;
        // N s_32_2: branch s_32_1 b35 b33
        if s_32_1 {
            return block_35(state, tracer, fn_state);
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
        // D s_34_0: read-var sv:u8
        let s_34_0: bool = fn_state.sv;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // D s_34_2: read-var tv:u8
        let s_34_2: bool = fn_state.tv;
        // D s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cast reint s_34_1 -> u128
        let s_34_4: u128 = (s_34_1.value() as u128);
        // D s_34_5: size-of s_34_1
        let s_34_5: u16 = s_34_1.length();
        // D s_34_6: cast reint s_34_3 -> u128
        let s_34_6: u128 = (s_34_3.value() as u128);
        // D s_34_7: size-of s_34_3
        let s_34_7: u16 = s_34_3.length();
        // D s_34_8: lsl s_34_4 s_34_7
        let s_34_8: u128 = s_34_4 << s_34_7;
        // D s_34_9: or s_34_8 s_34_6
        let s_34_9: u128 = ((s_34_8) | (s_34_6));
        // D s_34_10: add s_34_5 s_34_7
        let s_34_10: u16 = (s_34_5 + s_34_7);
        // D s_34_11: create-bits s_34_9 s_34_10
        let s_34_11: Bits = Bits::new(s_34_9, s_34_10);
        // D s_34_12: cast reint s_34_11 -> u8
        let s_34_12: u8 = (s_34_11.value() as u8);
        // D s_34_13: read-var ccu:u8
        let s_34_13: bool = fn_state.ccu;
        // D s_34_14: read-var cc:u14
        let s_34_14: u16 = fn_state.cc;
        // D s_34_15: read-var lastfailed:u8
        let s_34_15: bool = fn_state.lastfailed;
        // D s_34_16: read-var transactional:u8
        let s_34_16: bool = fn_state.transactional;
        // C s_34_17: const #7u : u8
        let s_34_17: u8 = 7;
        // D s_34_18: read-var el:u8
        let s_34_18: u8 = fn_state.el;
        // D s_34_19: read-var mispredict:u8
        let s_34_19: bool = fn_state.mispredict;
        // D s_34_20: read-var source_address:u64
        let s_34_20: u64 = fn_state.source_address;
        // D s_34_21: read-var target_address:u64
        let s_34_21: u64 = fn_state.target_address;
        // D s_34_22: call UpdateBranchRecordBuffer(s_34_13, s_34_14, s_34_15, s_34_16, s_34_17, s_34_18, s_34_19, s_34_12, s_34_20, s_34_21)
        let s_34_22: () = UpdateBranchRecordBuffer(
            state,
            tracer,
            s_34_13,
            s_34_14,
            s_34_15,
            s_34_16,
            s_34_17,
            s_34_18,
            s_34_19,
            s_34_12,
            s_34_20,
            s_34_21,
        );
        // C s_34_23: const #16536u : u32
        let s_34_23: u32 = 16536;
        // D s_34_24: read-reg s_34_23:struct
        let s_34_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_34_23 as isize);
            tracer.read_register(s_34_23 as isize, value);
            value
        };
        // C s_34_25: const #16536u : u32
        let s_34_25: u32 = 16536;
        // N s_34_26: write-reg s_34_25 <= s_34_24
        let s_34_26: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_34_25 as isize, s_34_24);
            tracer.write_register(s_34_25 as isize, s_34_24);
        };
        // C s_34_27: const #216u : u32
        let s_34_27: u32 = 216;
        // D s_34_28: read-reg s_34_27:u16
        let s_34_28: u16 = {
            let value = state.read_register::<u16>(s_34_27 as isize);
            tracer.read_register(s_34_27 as isize, value);
            value
        };
        // D s_34_29: call PMUEvent(s_34_28)
        let s_34_29: () = PMUEvent(state, tracer, s_34_28);
        // N s_34_30: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #64s : i
        let s_35_0: i128 = 64;
        // S s_35_1: call Zeros(s_35_0)
        let s_35_1: Bits = Zeros(state, tracer, s_35_0);
        // S s_35_2: cast reint s_35_1 -> u64
        let s_35_2: u64 = (s_35_1.value() as u64);
        // D s_35_3: write-var target_address <= s_35_2
        fn_state.target_address = s_35_2;
        // N s_35_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call PC_read(s_36_0)
        let s_36_1: u64 = PC_read(state, tracer, s_36_0);
        // D s_36_2: write-var source_address <= s_36_1
        fn_state.source_address = s_36_1;
        // N s_36_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var tv <= s_37_0
        fn_state.tv = s_37_0;
        // N s_37_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var sv <= s_38_0
        fn_state.sv = s_38_0;
        // N s_38_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var mispredict <= s_39_0
        fn_state.mispredict = s_39_0;
        // N s_39_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call BranchMispredict(s_40_0)
        let s_40_1: bool = BranchMispredict(state, tracer, s_40_0);
        // D s_40_2: write-var gs#26321 <= s_40_1
        fn_state.gs_26321 = s_40_1;
        // N s_40_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call BRBEMispredictAllowed(s_41_0)
        let s_41_1: bool = BRBEMispredictAllowed(state, tracer, s_41_0);
        // D s_41_2: write-var gs#26320 <= s_41_1
        fn_state.gs_26320 = s_41_1;
        // N s_41_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #16975u : u32
        let s_42_0: u32 = 16975;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: write-var el <= s_42_1
        fn_state.el = s_42_1;
        // N s_42_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var transactional <= s_43_0
        fn_state.transactional = s_43_0;
        // N s_43_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #100180u : u32
        let s_44_0: u32 = 100180;
        // D s_44_1: read-reg s_44_0:i
        let s_44_1: i128 = {
            let value = state.read_register::<i128>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // C s_44_2: const #0s : i
        let s_44_2: i128 = 0;
        // D s_44_3: cmp-gt s_44_1 s_44_2
        let s_44_3: bool = ((s_44_1) > (s_44_2));
        // D s_44_4: write-var gs#26319 <= s_44_3
        fn_state.gs_26319 = s_44_3;
        // N s_44_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call HaveTME(s_45_0)
        let s_45_1: bool = HaveTME(state, tracer, s_45_0);
        // D s_45_2: write-var gs#26317 <= s_45_1
        fn_state.gs_26317 = s_45_1;
        // N s_45_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #16536u : u32
        let s_46_0: u32 = 16536;
        // D s_46_1: read-reg s_46_0:struct
        let s_46_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call _get_BRBFCR_EL1_Type_LASTFAILED(s_46_1)
        let s_46_2: bool = u_get_BRBFCR_EL1_Type_LASTFAILED(state, tracer, s_46_1);
        // D s_46_3: write-var lastfailed <= s_46_2
        fn_state.lastfailed = s_46_2;
        // N s_46_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var gs#26310 <= s_47_0
        fn_state.gs_26310 = s_47_0;
        // N s_47_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#26301 <= s_49_0
        fn_state.gs_26301 = s_49_0;
        // N s_49_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var source_el:u8
        let s_50_0: u8 = fn_state.source_el;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 2u16);
        // C s_50_2: const #432u : u32
        let s_50_2: u32 = 432;
        // D s_50_3: read-reg s_50_2:u8
        let s_50_3: u8 = {
            let value = state.read_register::<u8>(s_50_2 as isize);
            tracer.read_register(s_50_2 as isize, value);
            value
        };
        // D s_50_4: cast zx s_50_3 -> bv
        let s_50_4: Bits = Bits::new(s_50_3 as u128, 2u16);
        // D s_50_5: cmp-eq s_50_1 s_50_4
        let s_50_5: bool = ((s_50_1) == (s_50_4));
        // D s_50_6: not s_50_5
        let s_50_6: bool = !s_50_5;
        // N s_50_7: branch s_50_6 b54 b51
        if s_50_6 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #18272u : u32
        let s_51_0: u32 = 18272;
        // D s_51_1: read-reg s_51_0:struct
        let s_51_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call _get_BRBCR_EL2_Type_ERTN(s_51_1)
        let s_51_2: bool = u_get_BRBCR_EL2_Type_ERTN(state, tracer, s_51_1);
        // D s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // C s_51_4: const #0u : u8
        let s_51_4: bool = false;
        // C s_51_5: cast zx s_51_4 -> bv
        let s_51_5: Bits = Bits::new(s_51_4 as u128, 1u16);
        // D s_51_6: cmp-eq s_51_3 s_51_5
        let s_51_6: bool = ((s_51_3) == (s_51_5));
        // N s_51_7: branch s_51_6 b53 b52
        if s_51_6 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var source_el:u8
        let s_54_0: u8 = fn_state.source_el;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 2u16);
        // C s_54_2: const #440u : u32
        let s_54_2: u32 = 440;
        // D s_54_3: read-reg s_54_2:u8
        let s_54_3: u8 = {
            let value = state.read_register::<u8>(s_54_2 as isize);
            tracer.read_register(s_54_2 as isize, value);
            value
        };
        // D s_54_4: cast zx s_54_3 -> bv
        let s_54_4: Bits = Bits::new(s_54_3 as u128, 2u16);
        // D s_54_5: cmp-eq s_54_1 s_54_4
        let s_54_5: bool = ((s_54_1) == (s_54_4));
        // D s_54_6: not s_54_5
        let s_54_6: bool = !s_54_5;
        // N s_54_7: branch s_54_6 b58 b55
        if s_54_6 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #90640u : u32
        let s_55_0: u32 = 90640;
        // D s_55_1: read-reg s_55_0:struct
        let s_55_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // D s_55_2: call _get_BRBCR_EL1_Type_ERTN(s_55_1)
        let s_55_2: bool = u_get_BRBCR_EL1_Type_ERTN(state, tracer, s_55_1);
        // D s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // C s_55_4: const #0u : u8
        let s_55_4: bool = false;
        // C s_55_5: cast zx s_55_4 -> bv
        let s_55_5: Bits = Bits::new(s_55_4 as u128, 1u16);
        // D s_55_6: cmp-eq s_55_3 s_55_5
        let s_55_6: bool = ((s_55_3) == (s_55_5));
        // N s_55_7: branch s_55_6 b57 b56
        if s_55_6 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: jump b5
        return block_5(state, tracer, fn_state);
    }
}
