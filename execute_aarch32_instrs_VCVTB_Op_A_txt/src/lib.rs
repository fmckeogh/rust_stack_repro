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
use D_set::*;
use FPSCR_read::*;
use CheckVFPEnabled::*;
use S_set::*;
use FPConvert__1::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VCVTB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    convert_from_half: bool,
    d: i64,
    lowbit: i64,
    m: i64,
    uses_double: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_893090: Bits,
        gs_893086: Bits,
        gs_893107: Bits,
        hpshadow_7402: u16,
        hp: u16,
        gs_893111: Bits,
        convert_from_half: bool,
        d: i64,
        lowbit: i64,
        m: i64,
        uses_double: bool,
    }
    let fn_state = FunctionState {
        convert_from_half,
        d,
        lowbit,
        m,
        uses_double,
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
        // D s_1_0: read-var convert_from_half:u8
        let s_1_0: bool = fn_state.convert_from_half;
        // N s_1_1: branch s_1_0 b8 b2
        if s_1_0 {
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
        // D s_2_0: read-var uses_double:u8
        let s_2_0: bool = fn_state.uses_double;
        // N s_2_1: branch s_2_0 b6 b3
        if s_2_0 {
            return block_6(state, tracer, fn_state);
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
        // D s_3_6: cast zx s_3_2 -> bv
        let s_3_6: Bits = Bits::new(s_3_2 as u128, 32u16);
        // D s_3_7: call FPConvert__1(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = FPConvert__1(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: write-var gs#893086 <= s_3_7
        fn_state.gs_893086 = s_3_7;
        // N s_3_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#893086:bv
        let s_4_0: Bits = fn_state.gs_893086;
        // D s_4_1: cast reint s_4_0 -> u16
        let s_4_1: u16 = (s_4_0.value() as u16);
        // D s_4_2: write-var hp <= s_4_1
        fn_state.hp = s_4_1;
        // N s_4_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var d:i64
        let s_5_0: i64 = fn_state.d;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call S_read(s_5_1)
        let s_5_2: u32 = S_read(state, tracer, s_5_1);
        // C s_5_3: const #15s : i
        let s_5_3: i128 = 15;
        // D s_5_4: read-var lowbit:i64
        let s_5_4: i64 = fn_state.lowbit;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: add s_5_5 s_5_3
        let s_5_6: i128 = (s_5_5 + s_5_3);
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // D s_5_8: cast zx s_5_2 -> bv
        let s_5_8: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_9: cast zx s_5_7 -> i
        let s_5_9: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_10: read-var lowbit:i64
        let s_5_10: i64 = fn_state.lowbit;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: read-var hp:u16
        let s_5_12: u16 = fn_state.hp;
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 16u16);
        // D s_5_14: sub s_5_9 s_5_11
        let s_5_14: i128 = ((s_5_9) - (s_5_11));
        // C s_5_15: const #1u : u64
        let s_5_15: u64 = 1;
        // C s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 64u16);
        // D s_5_17: lsl s_5_16 s_5_14
        let s_5_17: Bits = s_5_16 << s_5_14;
        // D s_5_18: sub s_5_17 s_5_16
        let s_5_18: Bits = ((s_5_17) - (s_5_16));
        // D s_5_19: and s_5_13 s_5_18
        let s_5_19: Bits = ((s_5_13) & (s_5_18));
        // D s_5_20: lsl s_5_19 s_5_11
        let s_5_20: Bits = s_5_19 << s_5_11;
        // D s_5_21: lsl s_5_18 s_5_11
        let s_5_21: Bits = s_5_18 << s_5_11;
        // D s_5_22: cmpl s_5_21
        let s_5_22: Bits = !s_5_21;
        // D s_5_23: and s_5_8 s_5_22
        let s_5_23: Bits = ((s_5_8) & (s_5_22));
        // D s_5_24: or s_5_23 s_5_20
        let s_5_24: Bits = ((s_5_23) | (s_5_20));
        // D s_5_25: cast reint s_5_24 -> u32
        let s_5_25: u32 = (s_5_24.value() as u32);
        // D s_5_26: read-var d:i64
        let s_5_26: i64 = fn_state.d;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: call S_set(s_5_27, s_5_25)
        let s_5_28: () = S_set(state, tracer, s_5_27, s_5_25);
        // N s_5_29: return
        return;
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
        // D s_6_2: call D_read(s_6_1)
        let s_6_2: u64 = D_read(state, tracer, s_6_1);
        // C s_6_3: const #() : ()
        let s_6_3: () = ();
        // S s_6_4: call FPSCR_read(s_6_3)
        let s_6_4: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_6_3);
        // C s_6_5: const #16s : i64
        let s_6_5: i64 = 16;
        // D s_6_6: cast zx s_6_2 -> bv
        let s_6_6: Bits = Bits::new(s_6_2 as u128, 64u16);
        // D s_6_7: call FPConvert__1(s_6_6, s_6_4, s_6_5)
        let s_6_7: Bits = FPConvert__1(state, tracer, s_6_6, s_6_4, s_6_5);
        // D s_6_8: write-var gs#893090 <= s_6_7
        fn_state.gs_893090 = s_6_7;
        // N s_6_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#893090:bv
        let s_7_0: Bits = fn_state.gs_893090;
        // D s_7_1: cast reint s_7_0 -> u16
        let s_7_1: u16 = (s_7_0.value() as u16);
        // D s_7_2: write-var hp <= s_7_1
        fn_state.hp = s_7_1;
        // N s_7_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var m:i64
        let s_8_0: i64 = fn_state.m;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call S_read(s_8_1)
        let s_8_2: u32 = S_read(state, tracer, s_8_1);
        // C s_8_3: const #16s : i
        let s_8_3: i128 = 16;
        // D s_8_4: cast zx s_8_2 -> bv
        let s_8_4: Bits = Bits::new(s_8_2 as u128, 32u16);
        // D s_8_5: read-var lowbit:i64
        let s_8_5: i64 = fn_state.lowbit;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: bit-extract s_8_4 s_8_6 s_8_3
        let s_8_7: Bits = (Bits::new(
            ((s_8_4) >> (s_8_6)).value(),
            u16::try_from(s_8_3).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u16
        let s_8_8: u16 = (s_8_7.value() as u16);
        // D s_8_9: write-var hpshadow#7402 <= s_8_8
        fn_state.hpshadow_7402 = s_8_8;
        // D s_8_10: read-var uses_double:u8
        let s_8_10: bool = fn_state.uses_double;
        // N s_8_11: branch s_8_10 b11 b9
        if s_8_10 {
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
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call FPSCR_read(s_9_0)
        let s_9_1: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_9_0);
        // C s_9_2: const #32s : i64
        let s_9_2: i64 = 32;
        // D s_9_3: read-var hpshadow#7402:u16
        let s_9_3: u16 = fn_state.hpshadow_7402;
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 16u16);
        // D s_9_5: call FPConvert__1(s_9_4, s_9_1, s_9_2)
        let s_9_5: Bits = FPConvert__1(state, tracer, s_9_4, s_9_1, s_9_2);
        // D s_9_6: write-var gs#893107 <= s_9_5
        fn_state.gs_893107 = s_9_5;
        // N s_9_7: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#893107:bv
        let s_10_0: Bits = fn_state.gs_893107;
        // D s_10_1: cast reint s_10_0 -> u32
        let s_10_1: u32 = (s_10_0.value() as u32);
        // D s_10_2: read-var d:i64
        let s_10_2: i64 = fn_state.d;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: call S_set(s_10_3, s_10_1)
        let s_10_4: () = S_set(state, tracer, s_10_3, s_10_1);
        // N s_10_5: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call FPSCR_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_11_0);
        // C s_11_2: const #64s : i64
        let s_11_2: i64 = 64;
        // D s_11_3: read-var hpshadow#7402:u16
        let s_11_3: u16 = fn_state.hpshadow_7402;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 16u16);
        // D s_11_5: call FPConvert__1(s_11_4, s_11_1, s_11_2)
        let s_11_5: Bits = FPConvert__1(state, tracer, s_11_4, s_11_1, s_11_2);
        // D s_11_6: write-var gs#893111 <= s_11_5
        fn_state.gs_893111 = s_11_5;
        // N s_11_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#893111:bv
        let s_12_0: Bits = fn_state.gs_893111;
        // D s_12_1: cast reint s_12_0 -> u64
        let s_12_1: u64 = (s_12_0.value() as u64);
        // D s_12_2: read-var d:i64
        let s_12_2: i64 = fn_state.d;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: call D_set(s_12_3, s_12_1)
        let s_12_4: () = D_set(state, tracer, s_12_3, s_12_1);
        // N s_12_5: return
        return;
    }
}
