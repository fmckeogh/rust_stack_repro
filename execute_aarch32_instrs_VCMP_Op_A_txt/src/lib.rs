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
use Mk_FPSCR_Type::*;
use FPCompare::*;
use FPSCR_read::*;
use FPSCR_read__1::*;
use FPSCR_write::*;
use FPZero::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VCMP_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i128,
    quiet_nan_exc: bool,
    with_zero: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        op64: u64,
        nzcv: u8,
        op16: u16,
        op32: u32,
        ga_352115: ProductType700c18a878c5601b,
        d: i64,
        esize: i64,
        m: i128,
        quiet_nan_exc: bool,
        with_zero: bool,
    }
    let fn_state = FunctionState {
        d,
        esize,
        m,
        quiet_nan_exc,
        with_zero,
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
        // D s_1_0: read-var esize:i64
        let s_1_0: i64 = fn_state.esize;
        // C s_1_1: const #16s : i
        let s_1_1: i128 = 16;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_1
        let s_1_3: bool = ((s_1_2) == (s_1_1));
        // D s_1_4: not s_1_3
        let s_1_4: bool = !s_1_3;
        // N s_1_5: branch s_1_4 b8 b2
        if s_1_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var with_zero:u8
        let s_2_0: bool = fn_state.with_zero;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var m:i
        let s_3_0: i128 = fn_state.m;
        // D s_3_1: call S_read(s_3_0)
        let s_3_1: u32 = S_read(state, tracer, s_3_0);
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: cast zx s_3_1 -> bv
        let s_3_3: Bits = Bits::new(s_3_1 as u128, 32u16);
        // C s_3_4: const #1s : i64
        let s_3_4: i64 = 1;
        // C s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // C s_3_6: const #15s : i
        let s_3_6: i128 = 15;
        // C s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: bit-extract s_3_3 s_3_2 s_3_7
        let s_3_8: Bits = (Bits::new(
            ((s_3_3) >> (s_3_2)).value(),
            u16::try_from(s_3_7).unwrap(),
        ));
        // D s_3_9: cast reint s_3_8 -> u16
        let s_3_9: u16 = (s_3_8.value() as u16);
        // D s_3_10: write-var op16 <= s_3_9
        fn_state.op16 = s_3_9;
        // N s_3_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var d:i64
        let s_4_0: i64 = fn_state.d;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call S_read(s_4_1)
        let s_4_2: u32 = S_read(state, tracer, s_4_1);
        // C s_4_3: const #0s : i
        let s_4_3: i128 = 0;
        // D s_4_4: cast zx s_4_2 -> bv
        let s_4_4: Bits = Bits::new(s_4_2 as u128, 32u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #15s : i
        let s_4_7: i128 = 15;
        // C s_4_8: add s_4_7 s_4_6
        let s_4_8: i128 = (s_4_7 + s_4_6);
        // D s_4_9: bit-extract s_4_4 s_4_3 s_4_8
        let s_4_9: Bits = (Bits::new(
            ((s_4_4) >> (s_4_3)).value(),
            u16::try_from(s_4_8).unwrap(),
        ));
        // D s_4_10: cast reint s_4_9 -> u16
        let s_4_10: u16 = (s_4_9.value() as u16);
        // C s_4_11: const #() : ()
        let s_4_11: () = ();
        // S s_4_12: call FPSCR_read(s_4_11)
        let s_4_12: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_4_11);
        // D s_4_13: cast zx s_4_10 -> bv
        let s_4_13: Bits = Bits::new(s_4_10 as u128, 16u16);
        // D s_4_14: read-var op16:u16
        let s_4_14: u16 = fn_state.op16;
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 16u16);
        // D s_4_16: read-var quiet_nan_exc:u8
        let s_4_16: bool = fn_state.quiet_nan_exc;
        // D s_4_17: call FPCompare(s_4_13, s_4_15, s_4_16, s_4_12)
        let s_4_17: u8 = FPCompare(state, tracer, s_4_13, s_4_15, s_4_16, s_4_12);
        // D s_4_18: write-var nzcv <= s_4_17
        fn_state.nzcv = s_4_17;
        // N s_4_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call FPSCR_read__1(s_6_0)
        let s_6_1: ProductType700c18a878c5601b = FPSCR_read__1(state, tracer, s_6_0);
        // D s_6_2: write-var ga#352115 <= s_6_1
        fn_state.ga_352115 = s_6_1;
        // D s_6_3: read-var ga#352115.0:struct
        let s_6_3: u32 = fn_state.ga_352115._0;
        // C s_6_4: const #28s : i
        let s_6_4: i128 = 28;
        // D s_6_5: cast zx s_6_3 -> bv
        let s_6_5: Bits = Bits::new(s_6_3 as u128, 32u16);
        // D s_6_6: read-var nzcv:u8
        let s_6_6: u8 = fn_state.nzcv;
        // D s_6_7: cast zx s_6_6 -> bv
        let s_6_7: Bits = Bits::new(s_6_6 as u128, 4u16);
        // C s_6_8: const #3s : i
        let s_6_8: i128 = 3;
        // C s_6_9: const #1u : u64
        let s_6_9: u64 = 1;
        // C s_6_10: cast zx s_6_9 -> bv
        let s_6_10: Bits = Bits::new(s_6_9 as u128, 64u16);
        // C s_6_11: lsl s_6_10 s_6_8
        let s_6_11: Bits = s_6_10 << s_6_8;
        // C s_6_12: sub s_6_11 s_6_10
        let s_6_12: Bits = ((s_6_11) - (s_6_10));
        // D s_6_13: and s_6_7 s_6_12
        let s_6_13: Bits = ((s_6_7) & (s_6_12));
        // D s_6_14: lsl s_6_13 s_6_4
        let s_6_14: Bits = s_6_13 << s_6_4;
        // C s_6_15: lsl s_6_12 s_6_4
        let s_6_15: Bits = s_6_12 << s_6_4;
        // C s_6_16: cmpl s_6_15
        let s_6_16: Bits = !s_6_15;
        // D s_6_17: and s_6_5 s_6_16
        let s_6_17: Bits = ((s_6_5) & (s_6_16));
        // D s_6_18: or s_6_17 s_6_14
        let s_6_18: Bits = ((s_6_17) | (s_6_14));
        // D s_6_19: cast reint s_6_18 -> u32
        let s_6_19: u32 = (s_6_18.value() as u32);
        // D s_6_20: call Mk_FPSCR_Type(s_6_19)
        let s_6_20: ProductType700c18a878c5601b = Mk_FPSCR_Type(state, tracer, s_6_19);
        // D s_6_21: call FPSCR_write(s_6_20)
        let s_6_21: () = FPSCR_write(state, tracer, s_6_20);
        // N s_6_22: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #16s : i64
        let s_7_0: i64 = 16;
        // C s_7_1: const #0u : u8
        let s_7_1: bool = false;
        // S s_7_2: call FPZero(s_7_1, s_7_0)
        let s_7_2: Bits = FPZero(state, tracer, s_7_1, s_7_0);
        // S s_7_3: cast reint s_7_2 -> u16
        let s_7_3: u16 = (s_7_2.value() as u16);
        // D s_7_4: write-var op16 <= s_7_3
        fn_state.op16 = s_7_3;
        // N s_7_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // C s_8_1: const #32s : i
        let s_8_1: i128 = 32;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b14 b9
        if s_8_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var with_zero:u8
        let s_9_0: bool = fn_state.with_zero;
        // N s_9_1: branch s_9_0 b13 b10
        if s_9_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var m:i
        let s_10_0: i128 = fn_state.m;
        // D s_10_1: call S_read(s_10_0)
        let s_10_1: u32 = S_read(state, tracer, s_10_0);
        // D s_10_2: write-var op32 <= s_10_1
        fn_state.op32 = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var d:i64
        let s_11_0: i64 = fn_state.d;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call S_read(s_11_1)
        let s_11_2: u32 = S_read(state, tracer, s_11_1);
        // C s_11_3: const #() : ()
        let s_11_3: () = ();
        // S s_11_4: call FPSCR_read(s_11_3)
        let s_11_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_11_3);
        // D s_11_5: cast zx s_11_2 -> bv
        let s_11_5: Bits = Bits::new(s_11_2 as u128, 32u16);
        // D s_11_6: read-var op32:u32
        let s_11_6: u32 = fn_state.op32;
        // D s_11_7: cast zx s_11_6 -> bv
        let s_11_7: Bits = Bits::new(s_11_6 as u128, 32u16);
        // D s_11_8: read-var quiet_nan_exc:u8
        let s_11_8: bool = fn_state.quiet_nan_exc;
        // D s_11_9: call FPCompare(s_11_5, s_11_7, s_11_8, s_11_4)
        let s_11_9: u8 = FPCompare(state, tracer, s_11_5, s_11_7, s_11_8, s_11_4);
        // D s_11_10: write-var nzcv <= s_11_9
        fn_state.nzcv = s_11_9;
        // N s_11_11: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #32s : i64
        let s_13_0: i64 = 32;
        // C s_13_1: const #0u : u8
        let s_13_1: bool = false;
        // S s_13_2: call FPZero(s_13_1, s_13_0)
        let s_13_2: Bits = FPZero(state, tracer, s_13_1, s_13_0);
        // S s_13_3: cast reint s_13_2 -> u32
        let s_13_3: u32 = (s_13_2.value() as u32);
        // D s_13_4: write-var op32 <= s_13_3
        fn_state.op32 = s_13_3;
        // N s_13_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esize:i64
        let s_14_0: i64 = fn_state.esize;
        // C s_14_1: const #64s : i
        let s_14_1: i128 = 64;
        // D s_14_2: cast zx s_14_0 -> i
        let s_14_2: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) == (s_14_1));
        // D s_14_4: not s_14_3
        let s_14_4: bool = !s_14_3;
        // N s_14_5: branch s_14_4 b20 b15
        if s_14_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var with_zero:u8
        let s_15_0: bool = fn_state.with_zero;
        // N s_15_1: branch s_15_0 b19 b16
        if s_15_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var m:i
        let s_16_0: i128 = fn_state.m;
        // D s_16_1: call D_read(s_16_0)
        let s_16_1: u64 = D_read(state, tracer, s_16_0);
        // D s_16_2: write-var op64 <= s_16_1
        fn_state.op64 = s_16_1;
        // N s_16_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var d:i64
        let s_17_0: i64 = fn_state.d;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call D_read(s_17_1)
        let s_17_2: u64 = D_read(state, tracer, s_17_1);
        // C s_17_3: const #() : ()
        let s_17_3: () = ();
        // S s_17_4: call FPSCR_read(s_17_3)
        let s_17_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_17_3);
        // D s_17_5: cast zx s_17_2 -> bv
        let s_17_5: Bits = Bits::new(s_17_2 as u128, 64u16);
        // D s_17_6: read-var op64:u64
        let s_17_6: u64 = fn_state.op64;
        // D s_17_7: cast zx s_17_6 -> bv
        let s_17_7: Bits = Bits::new(s_17_6 as u128, 64u16);
        // D s_17_8: read-var quiet_nan_exc:u8
        let s_17_8: bool = fn_state.quiet_nan_exc;
        // D s_17_9: call FPCompare(s_17_5, s_17_7, s_17_8, s_17_4)
        let s_17_9: u8 = FPCompare(state, tracer, s_17_5, s_17_7, s_17_8, s_17_4);
        // D s_17_10: write-var nzcv <= s_17_9
        fn_state.nzcv = s_17_9;
        // N s_17_11: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // C s_19_1: const #0u : u8
        let s_19_1: bool = false;
        // S s_19_2: call FPZero(s_19_1, s_19_0)
        let s_19_2: Bits = FPZero(state, tracer, s_19_1, s_19_0);
        // S s_19_3: cast reint s_19_2 -> u64
        let s_19_3: u64 = (s_19_2.value() as u64);
        // D s_19_4: write-var op64 <= s_19_3
        fn_state.op64 = s_19_3;
        // N s_19_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b6
        return block_6(state, tracer, fn_state);
    }
}
