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
use CheckVFPEnabled::*;
use D_set::*;
use FPSCR_read::*;
use FPToFixed::*;
use Zeros::*;
use S_set::*;
use S_read::*;
use D_read::*;
use FixedToFP::*;
use common::*;
pub fn execute_aarch32_instrs_VCVT_iv_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i64,
    rounding: u32,
    to_integer: bool,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_893425: Bits,
        gs_893431: Bits,
        gs_893415: Bits,
        gs_893440: Bits,
        gs_893445: Bits,
        gs_893451: Bits,
        d: i64,
        esize: i64,
        m: i64,
        rounding: u32,
        to_integer: bool,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        esize,
        m,
        rounding,
        to_integer,
        is_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var to_integer:u8
        let s_1_0: bool = fn_state.to_integer;
        // N s_1_1: branch s_1_0 b12 b2
        if s_1_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esize:i64
        let s_2_0: i64 = fn_state.esize;
        // C s_2_1: const #16s : i
        let s_2_1: i128 = 16;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) == (s_2_1));
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b5 b3
        if s_2_4 {
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
        // D s_3_0: read-var m:i64
        let s_3_0: i64 = fn_state.m;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call S_read(s_3_1)
        let s_3_2: u32 = S_read(state, tracer, s_3_1);
        // C s_3_3: const #() : ()
        let s_3_3: () = ();
        // S s_3_4: call FPSCR_read(s_3_3)
        let s_3_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_3_3);
        // C s_3_5: const #16s : i64
        let s_3_5: i64 = 16;
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // D s_3_7: cast zx s_3_2 -> bv
        let s_3_7: Bits = Bits::new(s_3_2 as u128, 32u16);
        // D s_3_8: read-var is_unsigned:u8
        let s_3_8: bool = fn_state.is_unsigned;
        // D s_3_9: read-var rounding:u32
        let s_3_9: u32 = fn_state.rounding;
        // D s_3_10: call FixedToFP(s_3_7, s_3_6, s_3_8, s_3_4, s_3_9, s_3_5)
        let s_3_10: Bits = FixedToFP(
            state,
            tracer,
            s_3_7,
            s_3_6,
            s_3_8,
            s_3_4,
            s_3_9,
            s_3_5,
        );
        // D s_3_11: write-var gs#893415 <= s_3_10
        fn_state.gs_893415 = s_3_10;
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#893415:bv
        let s_4_0: Bits = fn_state.gs_893415;
        // D s_4_1: cast reint s_4_0 -> u16
        let s_4_1: u16 = (s_4_0.value() as u16);
        // C s_4_2: const #16s : i
        let s_4_2: i128 = 16;
        // S s_4_3: call Zeros(s_4_2)
        let s_4_3: Bits = Zeros(state, tracer, s_4_2);
        // S s_4_4: cast reint s_4_3 -> u16
        let s_4_4: u16 = (s_4_3.value() as u16);
        // S s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 16u16);
        // D s_4_6: cast zx s_4_1 -> bv
        let s_4_6: Bits = Bits::new(s_4_1 as u128, 16u16);
        // S s_4_7: cast reint s_4_5 -> u128
        let s_4_7: u128 = (s_4_5.value() as u128);
        // D s_4_8: size-of s_4_5
        let s_4_8: u16 = s_4_5.length();
        // D s_4_9: cast reint s_4_6 -> u128
        let s_4_9: u128 = (s_4_6.value() as u128);
        // D s_4_10: size-of s_4_6
        let s_4_10: u16 = s_4_6.length();
        // D s_4_11: lsl s_4_7 s_4_10
        let s_4_11: u128 = s_4_7 << s_4_10;
        // D s_4_12: or s_4_11 s_4_9
        let s_4_12: u128 = ((s_4_11) | (s_4_9));
        // D s_4_13: add s_4_8 s_4_10
        let s_4_13: u16 = (s_4_8 + s_4_10);
        // D s_4_14: create-bits s_4_12 s_4_13
        let s_4_14: Bits = Bits::new(s_4_12, s_4_13);
        // D s_4_15: cast reint s_4_14 -> u32
        let s_4_15: u32 = (s_4_14.value() as u32);
        // D s_4_16: read-var d:i64
        let s_4_16: i64 = fn_state.d;
        // D s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // D s_4_18: call S_set(s_4_17, s_4_15)
        let s_4_18: () = S_set(state, tracer, s_4_17, s_4_15);
        // N s_4_19: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esize:i64
        let s_5_0: i64 = fn_state.esize;
        // C s_5_1: const #32s : i
        let s_5_1: i128 = 32;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b8 b6
        if s_5_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var m:i64
        let s_6_0: i64 = fn_state.m;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call S_read(s_6_1)
        let s_6_2: u32 = S_read(state, tracer, s_6_1);
        // C s_6_3: const #() : ()
        let s_6_3: () = ();
        // S s_6_4: call FPSCR_read(s_6_3)
        let s_6_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_6_3);
        // C s_6_5: const #32s : i64
        let s_6_5: i64 = 32;
        // C s_6_6: const #0s : i
        let s_6_6: i128 = 0;
        // D s_6_7: cast zx s_6_2 -> bv
        let s_6_7: Bits = Bits::new(s_6_2 as u128, 32u16);
        // D s_6_8: read-var is_unsigned:u8
        let s_6_8: bool = fn_state.is_unsigned;
        // D s_6_9: read-var rounding:u32
        let s_6_9: u32 = fn_state.rounding;
        // D s_6_10: call FixedToFP(s_6_7, s_6_6, s_6_8, s_6_4, s_6_9, s_6_5)
        let s_6_10: Bits = FixedToFP(
            state,
            tracer,
            s_6_7,
            s_6_6,
            s_6_8,
            s_6_4,
            s_6_9,
            s_6_5,
        );
        // D s_6_11: write-var gs#893425 <= s_6_10
        fn_state.gs_893425 = s_6_10;
        // N s_6_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#893425:bv
        let s_7_0: Bits = fn_state.gs_893425;
        // D s_7_1: cast reint s_7_0 -> u32
        let s_7_1: u32 = (s_7_0.value() as u32);
        // D s_7_2: read-var d:i64
        let s_7_2: i64 = fn_state.d;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: call S_set(s_7_3, s_7_1)
        let s_7_4: () = S_set(state, tracer, s_7_3, s_7_1);
        // N s_7_5: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // C s_8_1: const #64s : i
        let s_8_1: i128 = 64;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b11 b9
        if s_8_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var m:i64
        let s_9_0: i64 = fn_state.m;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call S_read(s_9_1)
        let s_9_2: u32 = S_read(state, tracer, s_9_1);
        // C s_9_3: const #() : ()
        let s_9_3: () = ();
        // S s_9_4: call FPSCR_read(s_9_3)
        let s_9_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_9_3);
        // C s_9_5: const #64s : i64
        let s_9_5: i64 = 64;
        // C s_9_6: const #0s : i
        let s_9_6: i128 = 0;
        // D s_9_7: cast zx s_9_2 -> bv
        let s_9_7: Bits = Bits::new(s_9_2 as u128, 32u16);
        // D s_9_8: read-var is_unsigned:u8
        let s_9_8: bool = fn_state.is_unsigned;
        // D s_9_9: read-var rounding:u32
        let s_9_9: u32 = fn_state.rounding;
        // D s_9_10: call FixedToFP(s_9_7, s_9_6, s_9_8, s_9_4, s_9_9, s_9_5)
        let s_9_10: Bits = FixedToFP(
            state,
            tracer,
            s_9_7,
            s_9_6,
            s_9_8,
            s_9_4,
            s_9_9,
            s_9_5,
        );
        // D s_9_11: write-var gs#893431 <= s_9_10
        fn_state.gs_893431 = s_9_10;
        // N s_9_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#893431:bv
        let s_10_0: Bits = fn_state.gs_893431;
        // D s_10_1: cast reint s_10_0 -> u64
        let s_10_1: u64 = (s_10_0.value() as u64);
        // D s_10_2: read-var d:i64
        let s_10_2: i64 = fn_state.d;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: call D_set(s_10_3, s_10_1)
        let s_10_4: () = D_set(state, tracer, s_10_3, s_10_1);
        // N s_10_5: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esize:i64
        let s_12_0: i64 = fn_state.esize;
        // C s_12_1: const #16s : i
        let s_12_1: i128 = 16;
        // D s_12_2: cast zx s_12_0 -> i
        let s_12_2: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_3: cmp-eq s_12_2 s_12_1
        let s_12_3: bool = ((s_12_2) == (s_12_1));
        // D s_12_4: not s_12_3
        let s_12_4: bool = !s_12_3;
        // N s_12_5: branch s_12_4 b15 b13
        if s_12_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var m:i64
        let s_13_0: i64 = fn_state.m;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call S_read(s_13_1)
        let s_13_2: u32 = S_read(state, tracer, s_13_1);
        // C s_13_3: const #0s : i
        let s_13_3: i128 = 0;
        // D s_13_4: cast zx s_13_2 -> bv
        let s_13_4: Bits = Bits::new(s_13_2 as u128, 32u16);
        // C s_13_5: const #1s : i64
        let s_13_5: i64 = 1;
        // C s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // C s_13_7: const #15s : i
        let s_13_7: i128 = 15;
        // C s_13_8: add s_13_7 s_13_6
        let s_13_8: i128 = (s_13_7 + s_13_6);
        // D s_13_9: bit-extract s_13_4 s_13_3 s_13_8
        let s_13_9: Bits = (Bits::new(
            ((s_13_4) >> (s_13_3)).value(),
            u16::try_from(s_13_8).unwrap(),
        ));
        // D s_13_10: cast reint s_13_9 -> u16
        let s_13_10: u16 = (s_13_9.value() as u16);
        // C s_13_11: const #() : ()
        let s_13_11: () = ();
        // S s_13_12: call FPSCR_read(s_13_11)
        let s_13_12: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_13_11);
        // C s_13_13: const #32s : i64
        let s_13_13: i64 = 32;
        // C s_13_14: const #0s : i
        let s_13_14: i128 = 0;
        // D s_13_15: cast zx s_13_10 -> bv
        let s_13_15: Bits = Bits::new(s_13_10 as u128, 16u16);
        // D s_13_16: read-var is_unsigned:u8
        let s_13_16: bool = fn_state.is_unsigned;
        // D s_13_17: read-var rounding:u32
        let s_13_17: u32 = fn_state.rounding;
        // D s_13_18: call FPToFixed(s_13_15, s_13_14, s_13_16, s_13_12, s_13_17, s_13_13)
        let s_13_18: Bits = FPToFixed(
            state,
            tracer,
            s_13_15,
            s_13_14,
            s_13_16,
            s_13_12,
            s_13_17,
            s_13_13,
        );
        // D s_13_19: write-var gs#893440 <= s_13_18
        fn_state.gs_893440 = s_13_18;
        // N s_13_20: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#893440:bv
        let s_14_0: Bits = fn_state.gs_893440;
        // D s_14_1: cast reint s_14_0 -> u32
        let s_14_1: u32 = (s_14_0.value() as u32);
        // D s_14_2: read-var d:i64
        let s_14_2: i64 = fn_state.d;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: call S_set(s_14_3, s_14_1)
        let s_14_4: () = S_set(state, tracer, s_14_3, s_14_1);
        // N s_14_5: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var esize:i64
        let s_15_0: i64 = fn_state.esize;
        // C s_15_1: const #32s : i
        let s_15_1: i128 = 32;
        // D s_15_2: cast zx s_15_0 -> i
        let s_15_2: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_3: cmp-eq s_15_2 s_15_1
        let s_15_3: bool = ((s_15_2) == (s_15_1));
        // D s_15_4: not s_15_3
        let s_15_4: bool = !s_15_3;
        // N s_15_5: branch s_15_4 b18 b16
        if s_15_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var m:i64
        let s_16_0: i64 = fn_state.m;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call S_read(s_16_1)
        let s_16_2: u32 = S_read(state, tracer, s_16_1);
        // C s_16_3: const #() : ()
        let s_16_3: () = ();
        // S s_16_4: call FPSCR_read(s_16_3)
        let s_16_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_16_3);
        // C s_16_5: const #32s : i64
        let s_16_5: i64 = 32;
        // C s_16_6: const #0s : i
        let s_16_6: i128 = 0;
        // D s_16_7: cast zx s_16_2 -> bv
        let s_16_7: Bits = Bits::new(s_16_2 as u128, 32u16);
        // D s_16_8: read-var is_unsigned:u8
        let s_16_8: bool = fn_state.is_unsigned;
        // D s_16_9: read-var rounding:u32
        let s_16_9: u32 = fn_state.rounding;
        // D s_16_10: call FPToFixed(s_16_7, s_16_6, s_16_8, s_16_4, s_16_9, s_16_5)
        let s_16_10: Bits = FPToFixed(
            state,
            tracer,
            s_16_7,
            s_16_6,
            s_16_8,
            s_16_4,
            s_16_9,
            s_16_5,
        );
        // D s_16_11: write-var gs#893445 <= s_16_10
        fn_state.gs_893445 = s_16_10;
        // N s_16_12: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#893445:bv
        let s_17_0: Bits = fn_state.gs_893445;
        // D s_17_1: cast reint s_17_0 -> u32
        let s_17_1: u32 = (s_17_0.value() as u32);
        // D s_17_2: read-var d:i64
        let s_17_2: i64 = fn_state.d;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: call S_set(s_17_3, s_17_1)
        let s_17_4: () = S_set(state, tracer, s_17_3, s_17_1);
        // N s_17_5: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var esize:i64
        let s_18_0: i64 = fn_state.esize;
        // C s_18_1: const #64s : i
        let s_18_1: i128 = 64;
        // D s_18_2: cast zx s_18_0 -> i
        let s_18_2: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_3: cmp-eq s_18_2 s_18_1
        let s_18_3: bool = ((s_18_2) == (s_18_1));
        // D s_18_4: not s_18_3
        let s_18_4: bool = !s_18_3;
        // N s_18_5: branch s_18_4 b21 b19
        if s_18_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var m:i64
        let s_19_0: i64 = fn_state.m;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call D_read(s_19_1)
        let s_19_2: u64 = D_read(state, tracer, s_19_1);
        // C s_19_3: const #() : ()
        let s_19_3: () = ();
        // S s_19_4: call FPSCR_read(s_19_3)
        let s_19_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_19_3);
        // C s_19_5: const #32s : i64
        let s_19_5: i64 = 32;
        // C s_19_6: const #0s : i
        let s_19_6: i128 = 0;
        // D s_19_7: cast zx s_19_2 -> bv
        let s_19_7: Bits = Bits::new(s_19_2 as u128, 64u16);
        // D s_19_8: read-var is_unsigned:u8
        let s_19_8: bool = fn_state.is_unsigned;
        // D s_19_9: read-var rounding:u32
        let s_19_9: u32 = fn_state.rounding;
        // D s_19_10: call FPToFixed(s_19_7, s_19_6, s_19_8, s_19_4, s_19_9, s_19_5)
        let s_19_10: Bits = FPToFixed(
            state,
            tracer,
            s_19_7,
            s_19_6,
            s_19_8,
            s_19_4,
            s_19_9,
            s_19_5,
        );
        // D s_19_11: write-var gs#893451 <= s_19_10
        fn_state.gs_893451 = s_19_10;
        // N s_19_12: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#893451:bv
        let s_20_0: Bits = fn_state.gs_893451;
        // D s_20_1: cast reint s_20_0 -> u32
        let s_20_1: u32 = (s_20_0.value() as u32);
        // D s_20_2: read-var d:i64
        let s_20_2: i64 = fn_state.d;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: call S_set(s_20_3, s_20_1)
        let s_20_4: () = S_set(state, tracer, s_20_3, s_20_1);
        // N s_20_5: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: return
        return;
    }
}
