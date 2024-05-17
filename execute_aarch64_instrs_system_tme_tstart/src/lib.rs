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
use TakeTransactionCheckpoint::*;
use FailTransaction::*;
use u_get_SCTLR_EL1_Type_TMT::*;
use u_get_SCTLR_EL2_Type_TMT0::*;
use NextInstrAddr::*;
use S1TranslationRegime__1::*;
use TransactionStartTrap::*;
use IsTMEEnabled::*;
use u_get_SCTLR_EL2_Type_TMT::*;
use u_get_SCTLR_EL1_Type_TME::*;
use HaveSME::*;
use X_set::*;
use Unreachable::*;
use u_get_SCTLR_EL1_Type_TMT0::*;
use u_get_SCTLR_EL1_Type_TME0::*;
use u_get_SCTLR_EL3_Type_TMT::*;
use u_get_SCTLR_EL2_Type_TME0::*;
use u_get_SCTLR_EL3_Type_TME::*;
use u_get_SCTLR_EL2_Type_TME::*;
use common::*;
pub fn execute_aarch64_instrs_system_tme_tstart<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_269989: u8,
        tme: bool,
        trivial: bool,
        VLshadow_2000: i64,
        gs_174056: bool,
        tmt: bool,
        VL: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#2000 <= s_0_2
        fn_state.VLshadow_2000 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call IsTMEEnabled(s_0_4)
        let s_0_5: bool = IsTMEEnabled(state, tracer, s_0_4);
        // S s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: branch s_0_6 b27 b1
        if s_0_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: write-var ga#269989 <= s_1_1
        fn_state.ga_269989 = s_1_1;
        // D s_1_3: read-var ga#269989:u8
        let s_1_3: u8 = fn_state.ga_269989;
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // C s_1_5: const #448u : u32
        let s_1_5: u32 = 448;
        // D s_1_6: read-reg s_1_5:u8
        let s_1_6: u8 = {
            let value = state.read_register::<u8>(s_1_5 as isize);
            tracer.read_register(s_1_5 as isize, value);
            value
        };
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 2u16);
        // D s_1_8: cmp-eq s_1_4 s_1_7
        let s_1_8: bool = ((s_1_4) == (s_1_7));
        // D s_1_9: not s_1_8
        let s_1_9: bool = !s_1_8;
        // N s_1_10: branch s_1_9 b20 b2
        if s_1_9 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call S1TranslationRegime__1(s_2_0)
        let s_2_1: u8 = S1TranslationRegime__1(state, tracer, s_2_0);
        // S s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
        // C s_2_3: const #440u : u32
        let s_2_3: u32 = 440;
        // D s_2_4: read-reg s_2_3:u8
        let s_2_4: u8 = {
            let value = state.read_register::<u8>(s_2_3 as isize);
            tracer.read_register(s_2_3 as isize, value);
            value
        };
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // D s_2_6: cmp-eq s_2_2 s_2_5
        let s_2_6: bool = ((s_2_2) == (s_2_5));
        // N s_2_7: branch s_2_6 b19 b3
        if s_2_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #20784u : u32
        let s_3_0: u32 = 20784;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_SCTLR_EL2_Type_TME0(s_3_1)
        let s_3_2: bool = u_get_SCTLR_EL2_Type_TME0(state, tracer, s_3_1);
        // D s_3_3: write-var tme <= s_3_2
        fn_state.tme = s_3_2;
        // C s_3_4: const #20784u : u32
        let s_3_4: u32 = 20784;
        // D s_3_5: read-reg s_3_4:struct
        let s_3_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_4 as isize);
            tracer.read_register(s_3_4 as isize, value);
            value
        };
        // D s_3_6: call _get_SCTLR_EL2_Type_TMT0(s_3_5)
        let s_3_6: bool = u_get_SCTLR_EL2_Type_TMT0(state, tracer, s_3_5);
        // D s_3_7: write-var tmt <= s_3_6
        fn_state.tmt = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var tme:u8
        let s_4_0: bool = fn_state.tme;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // D s_4_5: read-var tmt:u8
        let s_4_5: bool = fn_state.tmt;
        // D s_4_6: cast zx s_4_5 -> bv
        let s_4_6: Bits = Bits::new(s_4_5 as u128, 1u16);
        // C s_4_7: const #1u : u8
        let s_4_7: bool = true;
        // C s_4_8: cast zx s_4_7 -> bv
        let s_4_8: Bits = Bits::new(s_4_7 as u128, 1u16);
        // D s_4_9: cmp-eq s_4_6 s_4_8
        let s_4_9: bool = ((s_4_6) == (s_4_8));
        // D s_4_10: write-var trivial <= s_4_9
        fn_state.trivial = s_4_9;
        // D s_4_11: not s_4_4
        let s_4_11: bool = !s_4_4;
        // N s_4_12: branch s_4_11 b18 b5
        if s_4_11 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var trivial:u8
        let s_5_0: bool = fn_state.trivial;
        // N s_5_1: branch s_5_0 b17 b6
        if s_5_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call HaveSME(s_6_0)
        let s_6_1: bool = HaveSME(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b16 b7
        if s_6_1 {
            return block_16(state, tracer, fn_state);
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
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#174056 <= s_7_0
        fn_state.gs_174056 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#174056:u8
        let s_8_0: bool = fn_state.gs_174056;
        // N s_8_1: branch s_8_0 b15 b9
        if s_8_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #100180u : u32
        let s_9_0: u32 = 100180;
        // D s_9_1: read-reg s_9_0:i
        let s_9_1: i128 = {
            let value = state.read_register::<i128>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #255s : i
        let s_9_2: i128 = 255;
        // D s_9_3: cmp-eq s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) == (s_9_2));
        // N s_9_4: branch s_9_3 b14 b10
        if s_9_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #100180u : u32
        let s_10_0: u32 = 100180;
        // D s_10_1: read-reg s_10_0:i
        let s_10_1: i128 = {
            let value = state.read_register::<i128>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #0s : i
        let s_10_2: i128 = 0;
        // D s_10_3: cmp-eq s_10_1 s_10_2
        let s_10_3: bool = ((s_10_1) == (s_10_2));
        // N s_10_4: branch s_10_3 b13 b11
        if s_10_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #100180u : u32
        let s_12_0: u32 = 100180;
        // D s_12_1: read-reg s_12_0:i
        let s_12_1: i128 = {
            let value = state.read_register::<i128>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #1s : i
        let s_12_2: i128 = 1;
        // D s_12_3: add s_12_1 s_12_2
        let s_12_3: i128 = (s_12_1 + s_12_2);
        // C s_12_4: const #100180u : u32
        let s_12_4: u32 = 100180;
        // N s_12_5: write-reg s_12_4 <= s_12_3
        let s_12_5: () = {
            state.write_register::<i128>(s_12_4 as isize, s_12_3);
            tracer.write_register(s_12_4 as isize, s_12_3);
        };
        // C s_12_6: const #64s : i64
        let s_12_6: i64 = 64;
        // D s_12_7: read-var t:i64
        let s_12_7: i64 = fn_state.t;
        // D s_12_8: cast zx s_12_7 -> i
        let s_12_8: i128 = (i128::try_from(s_12_7).unwrap());
        // C s_12_9: const #0u : u64
        let s_12_9: u64 = 0;
        // C s_12_10: cast zx s_12_9 -> bv
        let s_12_10: Bits = Bits::new(s_12_9 as u128, 64u16);
        // D s_12_11: call X_set(s_12_8, s_12_6, s_12_10)
        let s_12_11: () = X_set(state, tracer, s_12_8, s_12_6, s_12_10);
        // N s_12_12: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // C s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // S s_13_2: call NextInstrAddr(s_13_1)
        let s_13_2: Bits = NextInstrAddr(state, tracer, s_13_1);
        // S s_13_3: cast reint s_13_2 -> u64
        let s_13_3: u64 = (s_13_2.value() as u64);
        // C s_13_4: const #100196u : u32
        let s_13_4: u32 = 100196;
        // N s_13_5: write-reg s_13_4 <= s_13_3
        let s_13_5: () = {
            state.write_register::<u64>(s_13_4 as isize, s_13_3);
            tracer.write_register(s_13_4 as isize, s_13_3);
        };
        // D s_13_6: read-var t:i64
        let s_13_6: i64 = fn_state.t;
        // D s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // C s_13_8: const #91716u : u32
        let s_13_8: u32 = 91716;
        // N s_13_9: write-reg s_13_8 <= s_13_7
        let s_13_9: () = {
            state.write_register::<i128>(s_13_8 as isize, s_13_7);
            tracer.write_register(s_13_8 as isize, s_13_7);
        };
        // D s_13_10: read-var VLshadow#2000:i64
        let s_13_10: i64 = fn_state.VLshadow_2000;
        // D s_13_11: cast zx s_13_10 -> i
        let s_13_11: i128 = (i128::try_from(s_13_10).unwrap());
        // D s_13_12: cast reint s_13_11 -> i64
        let s_13_12: i64 = (s_13_11 as i64);
        // C s_13_13: const #8s : i
        let s_13_13: i128 = 8;
        // D s_13_14: read-var VLshadow#2000:i64
        let s_13_14: i64 = fn_state.VLshadow_2000;
        // D s_13_15: cast zx s_13_14 -> i
        let s_13_15: i128 = (i128::try_from(s_13_14).unwrap());
        // D s_13_16: div s_13_15 s_13_13
        let s_13_16: i128 = ((s_13_15) / (s_13_13));
        // D s_13_17: cast reint s_13_16 -> i64
        let s_13_17: i64 = (s_13_16 as i64);
        // D s_13_18: cast zx s_13_17 -> i
        let s_13_18: i128 = (i128::try_from(s_13_17).unwrap());
        // D s_13_19: cast reint s_13_18 -> i64
        let s_13_19: i64 = (s_13_18 as i64);
        // D s_13_20: cast zx s_13_12 -> i
        let s_13_20: i128 = (i128::try_from(s_13_12).unwrap());
        // D s_13_21: cast zx s_13_19 -> i
        let s_13_21: i128 = (i128::try_from(s_13_19).unwrap());
        // D s_13_22: call TakeTransactionCheckpoint(s_13_20, s_13_21)
        let s_13_22: () = TakeTransactionCheckpoint(state, tracer, s_13_20, s_13_21);
        // N s_13_23: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #3u : u32
        let s_14_0: u32 = 3;
        // C s_14_1: const #0u : u8
        let s_14_1: bool = false;
        // S s_14_2: call FailTransaction(s_14_0, s_14_1)
        let s_14_2: () = FailTransaction(state, tracer, s_14_0, s_14_1);
        // N s_14_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #2u : u32
        let s_15_0: u32 = 2;
        // C s_15_1: const #0u : u8
        let s_15_1: bool = false;
        // S s_15_2: call FailTransaction(s_15_0, s_15_1)
        let s_15_2: () = FailTransaction(state, tracer, s_15_0, s_15_1);
        // N s_15_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #16989u : u32
        let s_16_0: u32 = 16989;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: bool = {
            let value = state.read_register::<bool>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 1u16);
        // C s_16_3: const #1u : u8
        let s_16_3: bool = true;
        // C s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 1u16);
        // D s_16_5: cmp-eq s_16_2 s_16_4
        let s_16_5: bool = ((s_16_2) == (s_16_4));
        // D s_16_6: write-var gs#174056 <= s_16_5
        fn_state.gs_174056 = s_16_5;
        // N s_16_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // C s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // S s_17_2: call NextInstrAddr(s_17_1)
        let s_17_2: Bits = NextInstrAddr(state, tracer, s_17_1);
        // S s_17_3: cast reint s_17_2 -> u64
        let s_17_3: u64 = (s_17_2.value() as u64);
        // C s_17_4: const #100196u : u32
        let s_17_4: u32 = 100196;
        // N s_17_5: write-reg s_17_4 <= s_17_3
        let s_17_5: () = {
            state.write_register::<u64>(s_17_4 as isize, s_17_3);
            tracer.write_register(s_17_4 as isize, s_17_3);
        };
        // D s_17_6: read-var t:i64
        let s_17_6: i64 = fn_state.t;
        // D s_17_7: cast zx s_17_6 -> i
        let s_17_7: i128 = (i128::try_from(s_17_6).unwrap());
        // C s_17_8: const #91716u : u32
        let s_17_8: u32 = 91716;
        // N s_17_9: write-reg s_17_8 <= s_17_7
        let s_17_9: () = {
            state.write_register::<i128>(s_17_8 as isize, s_17_7);
            tracer.write_register(s_17_8 as isize, s_17_7);
        };
        // C s_17_10: const #6u : u32
        let s_17_10: u32 = 6;
        // C s_17_11: const #0u : u8
        let s_17_11: bool = false;
        // S s_17_12: call FailTransaction(s_17_10, s_17_11)
        let s_17_12: () = FailTransaction(state, tracer, s_17_10, s_17_11);
        // N s_17_13: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var t:i64
        let s_18_0: i64 = fn_state.t;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call TransactionStartTrap(s_18_1)
        let s_18_2: () = TransactionStartTrap(state, tracer, s_18_1);
        // N s_18_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #90272u : u32
        let s_19_0: u32 = 90272;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_SCTLR_EL1_Type_TME0(s_19_1)
        let s_19_2: bool = u_get_SCTLR_EL1_Type_TME0(state, tracer, s_19_1);
        // D s_19_3: write-var tme <= s_19_2
        fn_state.tme = s_19_2;
        // C s_19_4: const #90272u : u32
        let s_19_4: u32 = 90272;
        // D s_19_5: read-reg s_19_4:struct
        let s_19_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_4 as isize);
            tracer.read_register(s_19_4 as isize, value);
            value
        };
        // D s_19_6: call _get_SCTLR_EL1_Type_TMT0(s_19_5)
        let s_19_6: bool = u_get_SCTLR_EL1_Type_TMT0(state, tracer, s_19_5);
        // D s_19_7: write-var tmt <= s_19_6
        fn_state.tmt = s_19_6;
        // N s_19_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var ga#269989:u8
        let s_20_0: u8 = fn_state.ga_269989;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 2u16);
        // C s_20_2: const #440u : u32
        let s_20_2: u32 = 440;
        // D s_20_3: read-reg s_20_2:u8
        let s_20_3: u8 = {
            let value = state.read_register::<u8>(s_20_2 as isize);
            tracer.read_register(s_20_2 as isize, value);
            value
        };
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 2u16);
        // D s_20_5: cmp-eq s_20_1 s_20_4
        let s_20_5: bool = ((s_20_1) == (s_20_4));
        // D s_20_6: not s_20_5
        let s_20_6: bool = !s_20_5;
        // N s_20_7: branch s_20_6 b22 b21
        if s_20_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #90272u : u32
        let s_21_0: u32 = 90272;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_SCTLR_EL1_Type_TME(s_21_1)
        let s_21_2: bool = u_get_SCTLR_EL1_Type_TME(state, tracer, s_21_1);
        // D s_21_3: write-var tme <= s_21_2
        fn_state.tme = s_21_2;
        // C s_21_4: const #90272u : u32
        let s_21_4: u32 = 90272;
        // D s_21_5: read-reg s_21_4:struct
        let s_21_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_4 as isize);
            tracer.read_register(s_21_4 as isize, value);
            value
        };
        // D s_21_6: call _get_SCTLR_EL1_Type_TMT(s_21_5)
        let s_21_6: bool = u_get_SCTLR_EL1_Type_TMT(state, tracer, s_21_5);
        // D s_21_7: write-var tmt <= s_21_6
        fn_state.tmt = s_21_6;
        // N s_21_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var ga#269989:u8
        let s_22_0: u8 = fn_state.ga_269989;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 2u16);
        // C s_22_2: const #432u : u32
        let s_22_2: u32 = 432;
        // D s_22_3: read-reg s_22_2:u8
        let s_22_3: u8 = {
            let value = state.read_register::<u8>(s_22_2 as isize);
            tracer.read_register(s_22_2 as isize, value);
            value
        };
        // D s_22_4: cast zx s_22_3 -> bv
        let s_22_4: Bits = Bits::new(s_22_3 as u128, 2u16);
        // D s_22_5: cmp-eq s_22_1 s_22_4
        let s_22_5: bool = ((s_22_1) == (s_22_4));
        // D s_22_6: not s_22_5
        let s_22_6: bool = !s_22_5;
        // N s_22_7: branch s_22_6 b24 b23
        if s_22_6 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #20784u : u32
        let s_23_0: u32 = 20784;
        // D s_23_1: read-reg s_23_0:struct
        let s_23_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call _get_SCTLR_EL2_Type_TME(s_23_1)
        let s_23_2: bool = u_get_SCTLR_EL2_Type_TME(state, tracer, s_23_1);
        // D s_23_3: write-var tme <= s_23_2
        fn_state.tme = s_23_2;
        // C s_23_4: const #20784u : u32
        let s_23_4: u32 = 20784;
        // D s_23_5: read-reg s_23_4:struct
        let s_23_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_23_4 as isize);
            tracer.read_register(s_23_4 as isize, value);
            value
        };
        // D s_23_6: call _get_SCTLR_EL2_Type_TMT(s_23_5)
        let s_23_6: bool = u_get_SCTLR_EL2_Type_TMT(state, tracer, s_23_5);
        // D s_23_7: write-var tmt <= s_23_6
        fn_state.tmt = s_23_6;
        // N s_23_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ga#269989:u8
        let s_24_0: u8 = fn_state.ga_269989;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 2u16);
        // C s_24_2: const #424u : u32
        let s_24_2: u32 = 424;
        // D s_24_3: read-reg s_24_2:u8
        let s_24_3: u8 = {
            let value = state.read_register::<u8>(s_24_2 as isize);
            tracer.read_register(s_24_2 as isize, value);
            value
        };
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 2u16);
        // D s_24_5: cmp-eq s_24_1 s_24_4
        let s_24_5: bool = ((s_24_1) == (s_24_4));
        // D s_24_6: not s_24_5
        let s_24_6: bool = !s_24_5;
        // N s_24_7: branch s_24_6 b26 b25
        if s_24_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #17072u : u32
        let s_25_0: u32 = 17072;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_SCTLR_EL3_Type_TME(s_25_1)
        let s_25_2: bool = u_get_SCTLR_EL3_Type_TME(state, tracer, s_25_1);
        // D s_25_3: write-var tme <= s_25_2
        fn_state.tme = s_25_2;
        // C s_25_4: const #17072u : u32
        let s_25_4: u32 = 17072;
        // D s_25_5: read-reg s_25_4:struct
        let s_25_5: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_4 as isize);
            tracer.read_register(s_25_4 as isize, value);
            value
        };
        // D s_25_6: call _get_SCTLR_EL3_Type_TMT(s_25_5)
        let s_25_6: bool = u_get_SCTLR_EL3_Type_TMT(state, tracer, s_25_5);
        // D s_25_7: write-var tmt <= s_25_6
        fn_state.tmt = s_25_6;
        // N s_25_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #() : ()
        let s_26_0: () = ();
        // S s_26_1: call Unreachable(s_26_0)
        let s_26_1: () = Unreachable(state, tracer, s_26_0);
        // N s_26_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
}
