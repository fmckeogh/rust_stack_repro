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
use Unreachable::*;
use PMUEvent::*;
use PC_read::*;
use BRBEMispredictAllowed::*;
use BranchMispredict::*;
use u_get_BRBFCR_EL1_Type_LASTFAILED::*;
use BranchRecordAllowed::*;
use FilterBranchRecord::*;
use UpdateBranchRecordBuffer::*;
use HaveTME::*;
use BranchEncCycleCount::*;
use common::*;
pub fn BRBEBranch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    br_type: u32,
    cond: bool,
    target_address: u64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        lastfailed: bool,
        ga_2107: ProductType1a93b8c16f53fb84,
        cc: u16,
        gs_3500: bool,
        branch_type: u8,
        transactional: bool,
        mispredict: bool,
        gs_3484: bool,
        ccu: bool,
        el: u8,
        branch_valid: bool,
        gs_3501: bool,
        br_type: u32,
        cond: bool,
        target_address: u64,
    }
    let fn_state = FunctionState {
        br_type,
        cond,
        target_address,
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
        // D s_0_2: call BranchRecordAllowed(s_0_1)
        let s_0_2: bool = BranchRecordAllowed(state, tracer, s_0_1);
        // D s_0_3: write-var branch_valid <= s_0_2
        fn_state.branch_valid = s_0_2;
        // D s_0_4: read-var branch_valid:u8
        let s_0_4: bool = fn_state.branch_valid;
        // N s_0_5: branch s_0_4 b35 b1
        if s_0_4 {
            return block_35(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#3484 <= s_1_0
        fn_state.gs_3484 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#3484:u8
        let s_2_0: bool = fn_state.gs_3484;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
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
        // D s_4_0: read-var branch_valid:u8
        let s_4_0: bool = fn_state.branch_valid;
        // C s_4_1: const #14032u : u32
        let s_4_1: u32 = 14032;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #5u : u32
        let s_5_0: u32 = 5;
        // D s_5_1: read-var br_type:u32
        let s_5_1: u32 = fn_state.br_type;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b26 b6
        if s_5_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var cond:u8
        let s_6_0: bool = fn_state.cond;
        // N s_6_1: branch s_6_0 b25 b7
        if s_6_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: u8 = 0;
        // D s_7_1: write-var branch_type <= s_7_0
        fn_state.branch_type = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
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
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call BranchEncCycleCount(s_9_0)
        let s_9_1: ProductType1a93b8c16f53fb84 = BranchEncCycleCount(
            state,
            tracer,
            s_9_0,
        );
        // D s_9_2: write-var ga#2107 <= s_9_1
        fn_state.ga_2107 = s_9_1;
        // D s_9_3: read-var ga#2107.0:struct
        let s_9_3: bool = fn_state.ga_2107._0;
        // D s_9_4: read-var ga#2107.1:struct
        let s_9_4: u16 = fn_state.ga_2107._1;
        // D s_9_5: write-var ccu <= s_9_3
        fn_state.ccu = s_9_3;
        // D s_9_6: write-var cc <= s_9_4
        fn_state.cc = s_9_4;
        // C s_9_7: const #() : ()
        let s_9_7: () = ();
        // S s_9_8: call HaveTME(s_9_7)
        let s_9_8: bool = HaveTME(state, tracer, s_9_7);
        // N s_9_9: branch s_9_8 b24 b10
        if s_9_8 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var lastfailed <= s_10_0
        fn_state.lastfailed = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveTME(s_11_0)
        let s_11_1: bool = HaveTME(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b23 b12
        if s_11_1 {
            return block_23(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#3500 <= s_12_0
        fn_state.gs_3500 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#3500:u8
        let s_13_0: bool = fn_state.gs_3500;
        // N s_13_1: branch s_13_0 b22 b14
        if s_13_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var transactional <= s_14_0
        fn_state.transactional = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
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
        // D s_15_2: write-var el <= s_15_1
        fn_state.el = s_15_1;
        // C s_15_3: const #() : ()
        let s_15_3: () = ();
        // S s_15_4: call BRBEMispredictAllowed(s_15_3)
        let s_15_4: bool = BRBEMispredictAllowed(state, tracer, s_15_3);
        // N s_15_5: branch s_15_4 b21 b16
        if s_15_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#3501 <= s_16_0
        fn_state.gs_3501 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#3501:u8
        let s_17_0: bool = fn_state.gs_3501;
        // N s_17_1: branch s_17_0 b20 b18
        if s_17_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var mispredict <= s_18_0
        fn_state.mispredict = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call PC_read(s_19_0)
        let s_19_1: u64 = PC_read(state, tracer, s_19_0);
        // D s_19_2: read-var ccu:u8
        let s_19_2: bool = fn_state.ccu;
        // D s_19_3: read-var cc:u14
        let s_19_3: u16 = fn_state.cc;
        // D s_19_4: read-var lastfailed:u8
        let s_19_4: bool = fn_state.lastfailed;
        // D s_19_5: read-var transactional:u8
        let s_19_5: bool = fn_state.transactional;
        // D s_19_6: read-var branch_type:u8
        let s_19_6: u8 = fn_state.branch_type;
        // D s_19_7: read-var el:u8
        let s_19_7: u8 = fn_state.el;
        // D s_19_8: read-var mispredict:u8
        let s_19_8: bool = fn_state.mispredict;
        // C s_19_9: const #3u : u8
        let s_19_9: u8 = 3;
        // D s_19_10: read-var target_address:u64
        let s_19_10: u64 = fn_state.target_address;
        // D s_19_11: call UpdateBranchRecordBuffer(s_19_2, s_19_3, s_19_4, s_19_5, s_19_6, s_19_7, s_19_8, s_19_9, s_19_1, s_19_10)
        let s_19_11: () = UpdateBranchRecordBuffer(
            state,
            tracer,
            s_19_2,
            s_19_3,
            s_19_4,
            s_19_5,
            s_19_6,
            s_19_7,
            s_19_8,
            s_19_9,
            s_19_1,
            s_19_10,
        );
        // C s_19_12: const #16536u : u32
        let s_19_12: u32 = 16536;
        // D s_19_13: read-reg s_19_12:struct
        let s_19_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_12 as isize);
            tracer.read_register(s_19_12 as isize, value);
            value
        };
        // C s_19_14: const #16536u : u32
        let s_19_14: u32 = 16536;
        // N s_19_15: write-reg s_19_14 <= s_19_13
        let s_19_15: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_19_14 as isize, s_19_13);
            tracer.write_register(s_19_14 as isize, s_19_13);
        };
        // C s_19_16: const #216u : u32
        let s_19_16: u32 = 216;
        // D s_19_17: read-reg s_19_16:u16
        let s_19_17: u16 = {
            let value = state.read_register::<u16>(s_19_16 as isize);
            tracer.read_register(s_19_16 as isize, value);
            value
        };
        // D s_19_18: call PMUEvent(s_19_17)
        let s_19_18: () = PMUEvent(state, tracer, s_19_17);
        // N s_19_19: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1u : u8
        let s_20_0: bool = true;
        // D s_20_1: write-var mispredict <= s_20_0
        fn_state.mispredict = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call BranchMispredict(s_21_0)
        let s_21_1: bool = BranchMispredict(state, tracer, s_21_0);
        // D s_21_2: write-var gs#3501 <= s_21_1
        fn_state.gs_3501 = s_21_1;
        // N s_21_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var transactional <= s_22_0
        fn_state.transactional = s_22_0;
        // N s_22_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #100180u : u32
        let s_23_0: u32 = 100180;
        // D s_23_1: read-reg s_23_0:i
        let s_23_1: i128 = {
            let value = state.read_register::<i128>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #0s : i
        let s_23_2: i128 = 0;
        // D s_23_3: cmp-gt s_23_1 s_23_2
        let s_23_3: bool = ((s_23_1) > (s_23_2));
        // D s_23_4: write-var gs#3500 <= s_23_3
        fn_state.gs_3500 = s_23_3;
        // N s_23_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #16536u : u32
        let s_24_0: u32 = 16536;
        // D s_24_1: read-reg s_24_0:struct
        let s_24_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call _get_BRBFCR_EL1_Type_LASTFAILED(s_24_1)
        let s_24_2: bool = u_get_BRBFCR_EL1_Type_LASTFAILED(state, tracer, s_24_1);
        // D s_24_3: write-var lastfailed <= s_24_2
        fn_state.lastfailed = s_24_2;
        // N s_24_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #8u : u8
        let s_25_0: u8 = 8;
        // D s_25_1: write-var branch_type <= s_25_0
        fn_state.branch_type = s_25_0;
        // N s_25_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #6u : u32
        let s_26_0: u32 = 6;
        // D s_26_1: read-var br_type:u32
        let s_26_1: u32 = fn_state.br_type;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: not s_26_2
        let s_26_3: bool = !s_26_2;
        // N s_26_4: branch s_26_3 b28 b27
        if s_26_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: u8 = 1;
        // D s_27_1: write-var branch_type <= s_27_0
        fn_state.branch_type = s_27_0;
        // N s_27_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u32
        let s_28_0: u32 = 0;
        // D s_28_1: read-var br_type:u32
        let s_28_1: u32 = fn_state.br_type;
        // D s_28_2: cmp-eq s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) == (s_28_1));
        // D s_28_3: not s_28_2
        let s_28_3: bool = !s_28_2;
        // N s_28_4: branch s_28_3 b30 b29
        if s_28_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #2u : u8
        let s_29_0: u8 = 2;
        // D s_29_1: write-var branch_type <= s_29_0
        fn_state.branch_type = s_29_0;
        // N s_29_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u32
        let s_30_0: u32 = 1;
        // D s_30_1: read-var br_type:u32
        let s_30_1: u32 = fn_state.br_type;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // N s_30_4: branch s_30_3 b32 b31
        if s_30_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #3u : u8
        let s_31_0: u8 = 3;
        // D s_31_1: write-var branch_type <= s_31_0
        fn_state.branch_type = s_31_0;
        // N s_31_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #4u : u32
        let s_32_0: u32 = 4;
        // D s_32_1: read-var br_type:u32
        let s_32_1: u32 = fn_state.br_type;
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // D s_32_3: not s_32_2
        let s_32_3: bool = !s_32_2;
        // N s_32_4: branch s_32_3 b34 b33
        if s_32_3 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #5u : u8
        let s_33_0: u8 = 5;
        // D s_33_1: write-var branch_type <= s_33_0
        fn_state.branch_type = s_33_0;
        // N s_33_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call Unreachable(s_34_0)
        let s_34_1: () = Unreachable(state, tracer, s_34_0);
        // N s_34_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var br_type:u32
        let s_35_0: u32 = fn_state.br_type;
        // D s_35_1: read-var cond:u8
        let s_35_1: bool = fn_state.cond;
        // D s_35_2: call FilterBranchRecord(s_35_0, s_35_1)
        let s_35_2: bool = FilterBranchRecord(state, tracer, s_35_0, s_35_1);
        // D s_35_3: write-var gs#3484 <= s_35_2
        fn_state.gs_3484 = s_35_2;
        // N s_35_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
