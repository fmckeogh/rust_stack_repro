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
use FPMul::*;
use FPAdd::*;
use FPNeg::*;
use FPSCR_read::*;
use D_set::*;
use Zeros::*;
use S_set::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VNMLA_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    vtype: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_901615: Bits,
        gs_901612: Bits,
        ga_357439: u16,
        gs_901627: Bits,
        gs_901569: Bits,
        gs_901610: Bits,
        product32shadow_7614: u32,
        gs_901623: Bits,
        gs_901602: Bits,
        gs_901606: Bits,
        gs_901574: Bits,
        gs_901619: Bits,
        gs_901648: Bits,
        ga_357451: u32,
        ga_357442: u16,
        gs_901644: Bits,
        gs_901587: Bits,
        gs_901640: Bits,
        gs_901635: Bits,
        gs_901584: Bits,
        product16shadow_7613: u16,
        gs_901597: Bits,
        gs_901563: Bits,
        product64shadow_7615: u64,
        gs_901631: Bits,
        gs_901637: Bits,
        gs_901572: Bits,
        ga_357464: u64,
        ga_357429: u16,
        ga_357432: u16,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        vtype: u32,
    }
    let fn_state = FunctionState {
        d,
        esize,
        m,
        n,
        vtype,
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
        // N s_1_5: branch s_1_4 b16 b2
        if s_1_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call S_read(s_2_1)
        let s_2_2: u32 = S_read(state, tracer, s_2_1);
        // C s_2_3: const #0s : i
        let s_2_3: i128 = 0;
        // D s_2_4: cast zx s_2_2 -> bv
        let s_2_4: Bits = Bits::new(s_2_2 as u128, 32u16);
        // C s_2_5: const #1s : i64
        let s_2_5: i64 = 1;
        // C s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // C s_2_7: const #15s : i
        let s_2_7: i128 = 15;
        // C s_2_8: add s_2_7 s_2_6
        let s_2_8: i128 = (s_2_7 + s_2_6);
        // D s_2_9: bit-extract s_2_4 s_2_3 s_2_8
        let s_2_9: Bits = (Bits::new(
            ((s_2_4) >> (s_2_3)).value(),
            u16::try_from(s_2_8).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u16
        let s_2_10: u16 = (s_2_9.value() as u16);
        // D s_2_11: read-var m:i64
        let s_2_11: i64 = fn_state.m;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: call S_read(s_2_12)
        let s_2_13: u32 = S_read(state, tracer, s_2_12);
        // C s_2_14: const #0s : i
        let s_2_14: i128 = 0;
        // D s_2_15: cast zx s_2_13 -> bv
        let s_2_15: Bits = Bits::new(s_2_13 as u128, 32u16);
        // C s_2_16: const #1s : i64
        let s_2_16: i64 = 1;
        // C s_2_17: cast zx s_2_16 -> i
        let s_2_17: i128 = (i128::try_from(s_2_16).unwrap());
        // C s_2_18: const #15s : i
        let s_2_18: i128 = 15;
        // C s_2_19: add s_2_18 s_2_17
        let s_2_19: i128 = (s_2_18 + s_2_17);
        // D s_2_20: bit-extract s_2_15 s_2_14 s_2_19
        let s_2_20: Bits = (Bits::new(
            ((s_2_15) >> (s_2_14)).value(),
            u16::try_from(s_2_19).unwrap(),
        ));
        // D s_2_21: cast reint s_2_20 -> u16
        let s_2_21: u16 = (s_2_20.value() as u16);
        // C s_2_22: const #() : ()
        let s_2_22: () = ();
        // S s_2_23: call FPSCR_read(s_2_22)
        let s_2_23: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_2_22);
        // D s_2_24: cast zx s_2_10 -> bv
        let s_2_24: Bits = Bits::new(s_2_10 as u128, 16u16);
        // D s_2_25: cast zx s_2_21 -> bv
        let s_2_25: Bits = Bits::new(s_2_21 as u128, 16u16);
        // D s_2_26: call FPMul(s_2_24, s_2_25, s_2_23)
        let s_2_26: Bits = FPMul(state, tracer, s_2_24, s_2_25, s_2_23);
        // D s_2_27: write-var gs#901563 <= s_2_26
        fn_state.gs_901563 = s_2_26;
        // N s_2_28: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#901563:bv
        let s_3_0: Bits = fn_state.gs_901563;
        // D s_3_1: cast reint s_3_0 -> u16
        let s_3_1: u16 = (s_3_0.value() as u16);
        // D s_3_2: write-var product16shadow#7613 <= s_3_1
        fn_state.product16shadow_7613 = s_3_1;
        // C s_3_3: const #0u : u32
        let s_3_3: u32 = 0;
        // D s_3_4: read-var vtype:u32
        let s_3_4: u32 = fn_state.vtype;
        // D s_3_5: cmp-eq s_3_3 s_3_4
        let s_3_5: bool = ((s_3_3) == (s_3_4));
        // D s_3_6: not s_3_5
        let s_3_6: bool = !s_3_5;
        // N s_3_7: branch s_3_6 b8 b4
        if s_3_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16s : i
        let s_4_0: i128 = 16;
        // S s_4_1: call Zeros(s_4_0)
        let s_4_1: Bits = Zeros(state, tracer, s_4_0);
        // S s_4_2: cast reint s_4_1 -> u16
        let s_4_2: u16 = (s_4_1.value() as u16);
        // D s_4_3: write-var ga#357432 <= s_4_2
        fn_state.ga_357432 = s_4_2;
        // D s_4_4: read-var d:i64
        let s_4_4: i64 = fn_state.d;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: call S_read(s_4_5)
        let s_4_6: u32 = S_read(state, tracer, s_4_5);
        // C s_4_7: const #0s : i
        let s_4_7: i128 = 0;
        // D s_4_8: cast zx s_4_6 -> bv
        let s_4_8: Bits = Bits::new(s_4_6 as u128, 32u16);
        // C s_4_9: const #1s : i64
        let s_4_9: i64 = 1;
        // C s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // C s_4_11: const #15s : i
        let s_4_11: i128 = 15;
        // C s_4_12: add s_4_11 s_4_10
        let s_4_12: i128 = (s_4_11 + s_4_10);
        // D s_4_13: bit-extract s_4_8 s_4_7 s_4_12
        let s_4_13: Bits = (Bits::new(
            ((s_4_8) >> (s_4_7)).value(),
            u16::try_from(s_4_12).unwrap(),
        ));
        // D s_4_14: cast reint s_4_13 -> u16
        let s_4_14: u16 = (s_4_13.value() as u16);
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 16u16);
        // D s_4_16: call FPNeg(s_4_15)
        let s_4_16: Bits = FPNeg(state, tracer, s_4_15);
        // D s_4_17: write-var gs#901569 <= s_4_16
        fn_state.gs_901569 = s_4_16;
        // N s_4_18: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#901569:bv
        let s_5_0: Bits = fn_state.gs_901569;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // D s_5_2: write-var ga#357429 <= s_5_1
        fn_state.ga_357429 = s_5_1;
        // D s_5_3: read-var product16shadow#7613:u16
        let s_5_3: u16 = fn_state.product16shadow_7613;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 16u16);
        // D s_5_5: call FPNeg(s_5_4)
        let s_5_5: Bits = FPNeg(state, tracer, s_5_4);
        // D s_5_6: write-var gs#901574 <= s_5_5
        fn_state.gs_901574 = s_5_5;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#901574:bv
        let s_6_0: Bits = fn_state.gs_901574;
        // D s_6_1: cast reint s_6_0 -> u16
        let s_6_1: u16 = (s_6_0.value() as u16);
        // C s_6_2: const #() : ()
        let s_6_2: () = ();
        // S s_6_3: call FPSCR_read(s_6_2)
        let s_6_3: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_6_2);
        // D s_6_4: read-var ga#357429:u16
        let s_6_4: u16 = fn_state.ga_357429;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 16u16);
        // D s_6_6: cast zx s_6_1 -> bv
        let s_6_6: Bits = Bits::new(s_6_1 as u128, 16u16);
        // D s_6_7: call FPAdd(s_6_5, s_6_6, s_6_3)
        let s_6_7: Bits = FPAdd(state, tracer, s_6_5, s_6_6, s_6_3);
        // D s_6_8: write-var gs#901572 <= s_6_7
        fn_state.gs_901572 = s_6_7;
        // N s_6_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#901572:bv
        let s_7_0: Bits = fn_state.gs_901572;
        // D s_7_1: cast reint s_7_0 -> u16
        let s_7_1: u16 = (s_7_0.value() as u16);
        // D s_7_2: read-var ga#357432:u16
        let s_7_2: u16 = fn_state.ga_357432;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 16u16);
        // D s_7_4: cast zx s_7_1 -> bv
        let s_7_4: Bits = Bits::new(s_7_1 as u128, 16u16);
        // D s_7_5: cast reint s_7_3 -> u128
        let s_7_5: u128 = (s_7_3.value() as u128);
        // D s_7_6: size-of s_7_3
        let s_7_6: u16 = s_7_3.length();
        // D s_7_7: cast reint s_7_4 -> u128
        let s_7_7: u128 = (s_7_4.value() as u128);
        // D s_7_8: size-of s_7_4
        let s_7_8: u16 = s_7_4.length();
        // D s_7_9: lsl s_7_5 s_7_8
        let s_7_9: u128 = s_7_5 << s_7_8;
        // D s_7_10: or s_7_9 s_7_7
        let s_7_10: u128 = ((s_7_9) | (s_7_7));
        // D s_7_11: add s_7_6 s_7_8
        let s_7_11: u16 = (s_7_6 + s_7_8);
        // D s_7_12: create-bits s_7_10 s_7_11
        let s_7_12: Bits = Bits::new(s_7_10, s_7_11);
        // D s_7_13: cast reint s_7_12 -> u32
        let s_7_13: u32 = (s_7_12.value() as u32);
        // D s_7_14: read-var d:i64
        let s_7_14: i64 = fn_state.d;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: call S_set(s_7_15, s_7_13)
        let s_7_16: () = S_set(state, tracer, s_7_15, s_7_13);
        // N s_7_17: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u32
        let s_8_0: u32 = 1;
        // D s_8_1: read-var vtype:u32
        let s_8_1: u32 = fn_state.vtype;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b12 b9
        if s_8_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #16s : i
        let s_9_0: i128 = 16;
        // S s_9_1: call Zeros(s_9_0)
        let s_9_1: Bits = Zeros(state, tracer, s_9_0);
        // S s_9_2: cast reint s_9_1 -> u16
        let s_9_2: u16 = (s_9_1.value() as u16);
        // D s_9_3: write-var ga#357439 <= s_9_2
        fn_state.ga_357439 = s_9_2;
        // D s_9_4: read-var d:i64
        let s_9_4: i64 = fn_state.d;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: call S_read(s_9_5)
        let s_9_6: u32 = S_read(state, tracer, s_9_5);
        // C s_9_7: const #0s : i
        let s_9_7: i128 = 0;
        // D s_9_8: cast zx s_9_6 -> bv
        let s_9_8: Bits = Bits::new(s_9_6 as u128, 32u16);
        // C s_9_9: const #1s : i64
        let s_9_9: i64 = 1;
        // C s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // C s_9_11: const #15s : i
        let s_9_11: i128 = 15;
        // C s_9_12: add s_9_11 s_9_10
        let s_9_12: i128 = (s_9_11 + s_9_10);
        // D s_9_13: bit-extract s_9_8 s_9_7 s_9_12
        let s_9_13: Bits = (Bits::new(
            ((s_9_8) >> (s_9_7)).value(),
            u16::try_from(s_9_12).unwrap(),
        ));
        // D s_9_14: cast reint s_9_13 -> u16
        let s_9_14: u16 = (s_9_13.value() as u16);
        // D s_9_15: cast zx s_9_14 -> bv
        let s_9_15: Bits = Bits::new(s_9_14 as u128, 16u16);
        // D s_9_16: call FPNeg(s_9_15)
        let s_9_16: Bits = FPNeg(state, tracer, s_9_15);
        // D s_9_17: write-var gs#901584 <= s_9_16
        fn_state.gs_901584 = s_9_16;
        // N s_9_18: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#901584:bv
        let s_10_0: Bits = fn_state.gs_901584;
        // D s_10_1: cast reint s_10_0 -> u16
        let s_10_1: u16 = (s_10_0.value() as u16);
        // C s_10_2: const #() : ()
        let s_10_2: () = ();
        // S s_10_3: call FPSCR_read(s_10_2)
        let s_10_3: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_10_2);
        // D s_10_4: cast zx s_10_1 -> bv
        let s_10_4: Bits = Bits::new(s_10_1 as u128, 16u16);
        // D s_10_5: read-var product16shadow#7613:u16
        let s_10_5: u16 = fn_state.product16shadow_7613;
        // D s_10_6: cast zx s_10_5 -> bv
        let s_10_6: Bits = Bits::new(s_10_5 as u128, 16u16);
        // D s_10_7: call FPAdd(s_10_4, s_10_6, s_10_3)
        let s_10_7: Bits = FPAdd(state, tracer, s_10_4, s_10_6, s_10_3);
        // D s_10_8: write-var gs#901587 <= s_10_7
        fn_state.gs_901587 = s_10_7;
        // N s_10_9: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#901587:bv
        let s_11_0: Bits = fn_state.gs_901587;
        // D s_11_1: cast reint s_11_0 -> u16
        let s_11_1: u16 = (s_11_0.value() as u16);
        // D s_11_2: read-var ga#357439:u16
        let s_11_2: u16 = fn_state.ga_357439;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 16u16);
        // D s_11_4: cast zx s_11_1 -> bv
        let s_11_4: Bits = Bits::new(s_11_1 as u128, 16u16);
        // D s_11_5: cast reint s_11_3 -> u128
        let s_11_5: u128 = (s_11_3.value() as u128);
        // D s_11_6: size-of s_11_3
        let s_11_6: u16 = s_11_3.length();
        // D s_11_7: cast reint s_11_4 -> u128
        let s_11_7: u128 = (s_11_4.value() as u128);
        // D s_11_8: size-of s_11_4
        let s_11_8: u16 = s_11_4.length();
        // D s_11_9: lsl s_11_5 s_11_8
        let s_11_9: u128 = s_11_5 << s_11_8;
        // D s_11_10: or s_11_9 s_11_7
        let s_11_10: u128 = ((s_11_9) | (s_11_7));
        // D s_11_11: add s_11_6 s_11_8
        let s_11_11: u16 = (s_11_6 + s_11_8);
        // D s_11_12: create-bits s_11_10 s_11_11
        let s_11_12: Bits = Bits::new(s_11_10, s_11_11);
        // D s_11_13: cast reint s_11_12 -> u32
        let s_11_13: u32 = (s_11_12.value() as u32);
        // D s_11_14: read-var d:i64
        let s_11_14: i64 = fn_state.d;
        // D s_11_15: cast zx s_11_14 -> i
        let s_11_15: i128 = (i128::try_from(s_11_14).unwrap());
        // D s_11_16: call S_set(s_11_15, s_11_13)
        let s_11_16: () = S_set(state, tracer, s_11_15, s_11_13);
        // N s_11_17: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2u : u32
        let s_12_0: u32 = 2;
        // D s_12_1: read-var vtype:u32
        let s_12_1: u32 = fn_state.vtype;
        // D s_12_2: cmp-eq s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) == (s_12_1));
        // D s_12_3: not s_12_2
        let s_12_3: bool = !s_12_2;
        // N s_12_4: branch s_12_3 b15 b13
        if s_12_3 {
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
        // C s_13_0: const #16s : i
        let s_13_0: i128 = 16;
        // S s_13_1: call Zeros(s_13_0)
        let s_13_1: Bits = Zeros(state, tracer, s_13_0);
        // S s_13_2: cast reint s_13_1 -> u16
        let s_13_2: u16 = (s_13_1.value() as u16);
        // D s_13_3: write-var ga#357442 <= s_13_2
        fn_state.ga_357442 = s_13_2;
        // D s_13_4: read-var product16shadow#7613:u16
        let s_13_4: u16 = fn_state.product16shadow_7613;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 16u16);
        // D s_13_6: call FPNeg(s_13_5)
        let s_13_6: Bits = FPNeg(state, tracer, s_13_5);
        // D s_13_7: write-var gs#901597 <= s_13_6
        fn_state.gs_901597 = s_13_6;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#901597:bv
        let s_14_0: Bits = fn_state.gs_901597;
        // D s_14_1: cast reint s_14_0 -> u16
        let s_14_1: u16 = (s_14_0.value() as u16);
        // D s_14_2: read-var ga#357442:u16
        let s_14_2: u16 = fn_state.ga_357442;
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 16u16);
        // D s_14_4: cast zx s_14_1 -> bv
        let s_14_4: Bits = Bits::new(s_14_1 as u128, 16u16);
        // D s_14_5: cast reint s_14_3 -> u128
        let s_14_5: u128 = (s_14_3.value() as u128);
        // D s_14_6: size-of s_14_3
        let s_14_6: u16 = s_14_3.length();
        // D s_14_7: cast reint s_14_4 -> u128
        let s_14_7: u128 = (s_14_4.value() as u128);
        // D s_14_8: size-of s_14_4
        let s_14_8: u16 = s_14_4.length();
        // D s_14_9: lsl s_14_5 s_14_8
        let s_14_9: u128 = s_14_5 << s_14_8;
        // D s_14_10: or s_14_9 s_14_7
        let s_14_10: u128 = ((s_14_9) | (s_14_7));
        // D s_14_11: add s_14_6 s_14_8
        let s_14_11: u16 = (s_14_6 + s_14_8);
        // D s_14_12: create-bits s_14_10 s_14_11
        let s_14_12: Bits = Bits::new(s_14_10, s_14_11);
        // D s_14_13: cast reint s_14_12 -> u32
        let s_14_13: u32 = (s_14_12.value() as u32);
        // D s_14_14: read-var d:i64
        let s_14_14: i64 = fn_state.d;
        // D s_14_15: cast zx s_14_14 -> i
        let s_14_15: i128 = (i128::try_from(s_14_14).unwrap());
        // D s_14_16: call S_set(s_14_15, s_14_13)
        let s_14_16: () = S_set(state, tracer, s_14_15, s_14_13);
        // N s_14_17: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esize:i64
        let s_16_0: i64 = fn_state.esize;
        // C s_16_1: const #32s : i
        let s_16_1: i128 = 32;
        // D s_16_2: cast zx s_16_0 -> i
        let s_16_2: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_1
        let s_16_3: bool = ((s_16_2) == (s_16_1));
        // D s_16_4: not s_16_3
        let s_16_4: bool = !s_16_3;
        // N s_16_5: branch s_16_4 b31 b17
        if s_16_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var n:i64
        let s_17_0: i64 = fn_state.n;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: call S_read(s_17_1)
        let s_17_2: u32 = S_read(state, tracer, s_17_1);
        // D s_17_3: read-var m:i64
        let s_17_3: i64 = fn_state.m;
        // D s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: call S_read(s_17_4)
        let s_17_5: u32 = S_read(state, tracer, s_17_4);
        // C s_17_6: const #() : ()
        let s_17_6: () = ();
        // S s_17_7: call FPSCR_read(s_17_6)
        let s_17_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_17_6);
        // D s_17_8: cast zx s_17_2 -> bv
        let s_17_8: Bits = Bits::new(s_17_2 as u128, 32u16);
        // D s_17_9: cast zx s_17_5 -> bv
        let s_17_9: Bits = Bits::new(s_17_5 as u128, 32u16);
        // D s_17_10: call FPMul(s_17_8, s_17_9, s_17_7)
        let s_17_10: Bits = FPMul(state, tracer, s_17_8, s_17_9, s_17_7);
        // D s_17_11: write-var gs#901602 <= s_17_10
        fn_state.gs_901602 = s_17_10;
        // N s_17_12: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#901602:bv
        let s_18_0: Bits = fn_state.gs_901602;
        // D s_18_1: cast reint s_18_0 -> u32
        let s_18_1: u32 = (s_18_0.value() as u32);
        // D s_18_2: write-var product32shadow#7614 <= s_18_1
        fn_state.product32shadow_7614 = s_18_1;
        // C s_18_3: const #0u : u32
        let s_18_3: u32 = 0;
        // D s_18_4: read-var vtype:u32
        let s_18_4: u32 = fn_state.vtype;
        // D s_18_5: cmp-eq s_18_3 s_18_4
        let s_18_5: bool = ((s_18_3) == (s_18_4));
        // D s_18_6: not s_18_5
        let s_18_6: bool = !s_18_5;
        // N s_18_7: branch s_18_6 b23 b19
        if s_18_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var d:i64
        let s_19_0: i64 = fn_state.d;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call S_read(s_19_1)
        let s_19_2: u32 = S_read(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 32u16);
        // D s_19_4: call FPNeg(s_19_3)
        let s_19_4: Bits = FPNeg(state, tracer, s_19_3);
        // D s_19_5: write-var gs#901606 <= s_19_4
        fn_state.gs_901606 = s_19_4;
        // N s_19_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#901606:bv
        let s_20_0: Bits = fn_state.gs_901606;
        // D s_20_1: cast reint s_20_0 -> u32
        let s_20_1: u32 = (s_20_0.value() as u32);
        // D s_20_2: write-var ga#357451 <= s_20_1
        fn_state.ga_357451 = s_20_1;
        // D s_20_3: read-var product32shadow#7614:u32
        let s_20_3: u32 = fn_state.product32shadow_7614;
        // D s_20_4: cast zx s_20_3 -> bv
        let s_20_4: Bits = Bits::new(s_20_3 as u128, 32u16);
        // D s_20_5: call FPNeg(s_20_4)
        let s_20_5: Bits = FPNeg(state, tracer, s_20_4);
        // D s_20_6: write-var gs#901612 <= s_20_5
        fn_state.gs_901612 = s_20_5;
        // N s_20_7: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#901612:bv
        let s_21_0: Bits = fn_state.gs_901612;
        // D s_21_1: cast reint s_21_0 -> u32
        let s_21_1: u32 = (s_21_0.value() as u32);
        // C s_21_2: const #() : ()
        let s_21_2: () = ();
        // S s_21_3: call FPSCR_read(s_21_2)
        let s_21_3: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_21_2);
        // D s_21_4: read-var ga#357451:u32
        let s_21_4: u32 = fn_state.ga_357451;
        // D s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 32u16);
        // D s_21_6: cast zx s_21_1 -> bv
        let s_21_6: Bits = Bits::new(s_21_1 as u128, 32u16);
        // D s_21_7: call FPAdd(s_21_5, s_21_6, s_21_3)
        let s_21_7: Bits = FPAdd(state, tracer, s_21_5, s_21_6, s_21_3);
        // D s_21_8: write-var gs#901610 <= s_21_7
        fn_state.gs_901610 = s_21_7;
        // N s_21_9: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#901610:bv
        let s_22_0: Bits = fn_state.gs_901610;
        // D s_22_1: cast reint s_22_0 -> u32
        let s_22_1: u32 = (s_22_0.value() as u32);
        // D s_22_2: read-var d:i64
        let s_22_2: i64 = fn_state.d;
        // D s_22_3: cast zx s_22_2 -> i
        let s_22_3: i128 = (i128::try_from(s_22_2).unwrap());
        // D s_22_4: call S_set(s_22_3, s_22_1)
        let s_22_4: () = S_set(state, tracer, s_22_3, s_22_1);
        // N s_22_5: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u32
        let s_23_0: u32 = 1;
        // D s_23_1: read-var vtype:u32
        let s_23_1: u32 = fn_state.vtype;
        // D s_23_2: cmp-eq s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) == (s_23_1));
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // N s_23_4: branch s_23_3 b27 b24
        if s_23_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var d:i64
        let s_24_0: i64 = fn_state.d;
        // D s_24_1: cast zx s_24_0 -> i
        let s_24_1: i128 = (i128::try_from(s_24_0).unwrap());
        // D s_24_2: call S_read(s_24_1)
        let s_24_2: u32 = S_read(state, tracer, s_24_1);
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 32u16);
        // D s_24_4: call FPNeg(s_24_3)
        let s_24_4: Bits = FPNeg(state, tracer, s_24_3);
        // D s_24_5: write-var gs#901615 <= s_24_4
        fn_state.gs_901615 = s_24_4;
        // N s_24_6: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#901615:bv
        let s_25_0: Bits = fn_state.gs_901615;
        // D s_25_1: cast reint s_25_0 -> u32
        let s_25_1: u32 = (s_25_0.value() as u32);
        // C s_25_2: const #() : ()
        let s_25_2: () = ();
        // S s_25_3: call FPSCR_read(s_25_2)
        let s_25_3: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_25_2);
        // D s_25_4: cast zx s_25_1 -> bv
        let s_25_4: Bits = Bits::new(s_25_1 as u128, 32u16);
        // D s_25_5: read-var product32shadow#7614:u32
        let s_25_5: u32 = fn_state.product32shadow_7614;
        // D s_25_6: cast zx s_25_5 -> bv
        let s_25_6: Bits = Bits::new(s_25_5 as u128, 32u16);
        // D s_25_7: call FPAdd(s_25_4, s_25_6, s_25_3)
        let s_25_7: Bits = FPAdd(state, tracer, s_25_4, s_25_6, s_25_3);
        // D s_25_8: write-var gs#901619 <= s_25_7
        fn_state.gs_901619 = s_25_7;
        // N s_25_9: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#901619:bv
        let s_26_0: Bits = fn_state.gs_901619;
        // D s_26_1: cast reint s_26_0 -> u32
        let s_26_1: u32 = (s_26_0.value() as u32);
        // D s_26_2: read-var d:i64
        let s_26_2: i64 = fn_state.d;
        // D s_26_3: cast zx s_26_2 -> i
        let s_26_3: i128 = (i128::try_from(s_26_2).unwrap());
        // D s_26_4: call S_set(s_26_3, s_26_1)
        let s_26_4: () = S_set(state, tracer, s_26_3, s_26_1);
        // N s_26_5: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #2u : u32
        let s_27_0: u32 = 2;
        // D s_27_1: read-var vtype:u32
        let s_27_1: u32 = fn_state.vtype;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // D s_27_3: not s_27_2
        let s_27_3: bool = !s_27_2;
        // N s_27_4: branch s_27_3 b30 b28
        if s_27_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var product32shadow#7614:u32
        let s_28_0: u32 = fn_state.product32shadow_7614;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 32u16);
        // D s_28_2: call FPNeg(s_28_1)
        let s_28_2: Bits = FPNeg(state, tracer, s_28_1);
        // D s_28_3: write-var gs#901623 <= s_28_2
        fn_state.gs_901623 = s_28_2;
        // N s_28_4: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#901623:bv
        let s_29_0: Bits = fn_state.gs_901623;
        // D s_29_1: cast reint s_29_0 -> u32
        let s_29_1: u32 = (s_29_0.value() as u32);
        // D s_29_2: read-var d:i64
        let s_29_2: i64 = fn_state.d;
        // D s_29_3: cast zx s_29_2 -> i
        let s_29_3: i128 = (i128::try_from(s_29_2).unwrap());
        // D s_29_4: call S_set(s_29_3, s_29_1)
        let s_29_4: () = S_set(state, tracer, s_29_3, s_29_1);
        // N s_29_5: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var esize:i64
        let s_31_0: i64 = fn_state.esize;
        // C s_31_1: const #64s : i
        let s_31_1: i128 = 64;
        // D s_31_2: cast zx s_31_0 -> i
        let s_31_2: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_3: cmp-eq s_31_2 s_31_1
        let s_31_3: bool = ((s_31_2) == (s_31_1));
        // D s_31_4: not s_31_3
        let s_31_4: bool = !s_31_3;
        // N s_31_5: branch s_31_4 b46 b32
        if s_31_4 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var n:i64
        let s_32_0: i64 = fn_state.n;
        // D s_32_1: cast zx s_32_0 -> i
        let s_32_1: i128 = (i128::try_from(s_32_0).unwrap());
        // D s_32_2: call D_read(s_32_1)
        let s_32_2: u64 = D_read(state, tracer, s_32_1);
        // D s_32_3: read-var m:i64
        let s_32_3: i64 = fn_state.m;
        // D s_32_4: cast zx s_32_3 -> i
        let s_32_4: i128 = (i128::try_from(s_32_3).unwrap());
        // D s_32_5: call D_read(s_32_4)
        let s_32_5: u64 = D_read(state, tracer, s_32_4);
        // C s_32_6: const #() : ()
        let s_32_6: () = ();
        // S s_32_7: call FPSCR_read(s_32_6)
        let s_32_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_32_6);
        // D s_32_8: cast zx s_32_2 -> bv
        let s_32_8: Bits = Bits::new(s_32_2 as u128, 64u16);
        // D s_32_9: cast zx s_32_5 -> bv
        let s_32_9: Bits = Bits::new(s_32_5 as u128, 64u16);
        // D s_32_10: call FPMul(s_32_8, s_32_9, s_32_7)
        let s_32_10: Bits = FPMul(state, tracer, s_32_8, s_32_9, s_32_7);
        // D s_32_11: write-var gs#901627 <= s_32_10
        fn_state.gs_901627 = s_32_10;
        // N s_32_12: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#901627:bv
        let s_33_0: Bits = fn_state.gs_901627;
        // D s_33_1: cast reint s_33_0 -> u64
        let s_33_1: u64 = (s_33_0.value() as u64);
        // D s_33_2: write-var product64shadow#7615 <= s_33_1
        fn_state.product64shadow_7615 = s_33_1;
        // C s_33_3: const #0u : u32
        let s_33_3: u32 = 0;
        // D s_33_4: read-var vtype:u32
        let s_33_4: u32 = fn_state.vtype;
        // D s_33_5: cmp-eq s_33_3 s_33_4
        let s_33_5: bool = ((s_33_3) == (s_33_4));
        // D s_33_6: not s_33_5
        let s_33_6: bool = !s_33_5;
        // N s_33_7: branch s_33_6 b38 b34
        if s_33_6 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var d:i64
        let s_34_0: i64 = fn_state.d;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call D_read(s_34_1)
        let s_34_2: u64 = D_read(state, tracer, s_34_1);
        // D s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 64u16);
        // D s_34_4: call FPNeg(s_34_3)
        let s_34_4: Bits = FPNeg(state, tracer, s_34_3);
        // D s_34_5: write-var gs#901631 <= s_34_4
        fn_state.gs_901631 = s_34_4;
        // N s_34_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#901631:bv
        let s_35_0: Bits = fn_state.gs_901631;
        // D s_35_1: cast reint s_35_0 -> u64
        let s_35_1: u64 = (s_35_0.value() as u64);
        // D s_35_2: write-var ga#357464 <= s_35_1
        fn_state.ga_357464 = s_35_1;
        // D s_35_3: read-var product64shadow#7615:u64
        let s_35_3: u64 = fn_state.product64shadow_7615;
        // D s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 64u16);
        // D s_35_5: call FPNeg(s_35_4)
        let s_35_5: Bits = FPNeg(state, tracer, s_35_4);
        // D s_35_6: write-var gs#901637 <= s_35_5
        fn_state.gs_901637 = s_35_5;
        // N s_35_7: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#901637:bv
        let s_36_0: Bits = fn_state.gs_901637;
        // D s_36_1: cast reint s_36_0 -> u64
        let s_36_1: u64 = (s_36_0.value() as u64);
        // C s_36_2: const #() : ()
        let s_36_2: () = ();
        // S s_36_3: call FPSCR_read(s_36_2)
        let s_36_3: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_36_2);
        // D s_36_4: read-var ga#357464:u64
        let s_36_4: u64 = fn_state.ga_357464;
        // D s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 64u16);
        // D s_36_6: cast zx s_36_1 -> bv
        let s_36_6: Bits = Bits::new(s_36_1 as u128, 64u16);
        // D s_36_7: call FPAdd(s_36_5, s_36_6, s_36_3)
        let s_36_7: Bits = FPAdd(state, tracer, s_36_5, s_36_6, s_36_3);
        // D s_36_8: write-var gs#901635 <= s_36_7
        fn_state.gs_901635 = s_36_7;
        // N s_36_9: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#901635:bv
        let s_37_0: Bits = fn_state.gs_901635;
        // D s_37_1: cast reint s_37_0 -> u64
        let s_37_1: u64 = (s_37_0.value() as u64);
        // D s_37_2: read-var d:i64
        let s_37_2: i64 = fn_state.d;
        // D s_37_3: cast zx s_37_2 -> i
        let s_37_3: i128 = (i128::try_from(s_37_2).unwrap());
        // D s_37_4: call D_set(s_37_3, s_37_1)
        let s_37_4: () = D_set(state, tracer, s_37_3, s_37_1);
        // N s_37_5: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u32
        let s_38_0: u32 = 1;
        // D s_38_1: read-var vtype:u32
        let s_38_1: u32 = fn_state.vtype;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // N s_38_4: branch s_38_3 b42 b39
        if s_38_3 {
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
        // D s_39_0: read-var d:i64
        let s_39_0: i64 = fn_state.d;
        // D s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // D s_39_2: call D_read(s_39_1)
        let s_39_2: u64 = D_read(state, tracer, s_39_1);
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 64u16);
        // D s_39_4: call FPNeg(s_39_3)
        let s_39_4: Bits = FPNeg(state, tracer, s_39_3);
        // D s_39_5: write-var gs#901640 <= s_39_4
        fn_state.gs_901640 = s_39_4;
        // N s_39_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#901640:bv
        let s_40_0: Bits = fn_state.gs_901640;
        // D s_40_1: cast reint s_40_0 -> u64
        let s_40_1: u64 = (s_40_0.value() as u64);
        // C s_40_2: const #() : ()
        let s_40_2: () = ();
        // S s_40_3: call FPSCR_read(s_40_2)
        let s_40_3: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_40_2);
        // D s_40_4: cast zx s_40_1 -> bv
        let s_40_4: Bits = Bits::new(s_40_1 as u128, 64u16);
        // D s_40_5: read-var product64shadow#7615:u64
        let s_40_5: u64 = fn_state.product64shadow_7615;
        // D s_40_6: cast zx s_40_5 -> bv
        let s_40_6: Bits = Bits::new(s_40_5 as u128, 64u16);
        // D s_40_7: call FPAdd(s_40_4, s_40_6, s_40_3)
        let s_40_7: Bits = FPAdd(state, tracer, s_40_4, s_40_6, s_40_3);
        // D s_40_8: write-var gs#901644 <= s_40_7
        fn_state.gs_901644 = s_40_7;
        // N s_40_9: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#901644:bv
        let s_41_0: Bits = fn_state.gs_901644;
        // D s_41_1: cast reint s_41_0 -> u64
        let s_41_1: u64 = (s_41_0.value() as u64);
        // D s_41_2: read-var d:i64
        let s_41_2: i64 = fn_state.d;
        // D s_41_3: cast zx s_41_2 -> i
        let s_41_3: i128 = (i128::try_from(s_41_2).unwrap());
        // D s_41_4: call D_set(s_41_3, s_41_1)
        let s_41_4: () = D_set(state, tracer, s_41_3, s_41_1);
        // N s_41_5: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #2u : u32
        let s_42_0: u32 = 2;
        // D s_42_1: read-var vtype:u32
        let s_42_1: u32 = fn_state.vtype;
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // D s_42_3: not s_42_2
        let s_42_3: bool = !s_42_2;
        // N s_42_4: branch s_42_3 b45 b43
        if s_42_3 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var product64shadow#7615:u64
        let s_43_0: u64 = fn_state.product64shadow_7615;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 64u16);
        // D s_43_2: call FPNeg(s_43_1)
        let s_43_2: Bits = FPNeg(state, tracer, s_43_1);
        // D s_43_3: write-var gs#901648 <= s_43_2
        fn_state.gs_901648 = s_43_2;
        // N s_43_4: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#901648:bv
        let s_44_0: Bits = fn_state.gs_901648;
        // D s_44_1: cast reint s_44_0 -> u64
        let s_44_1: u64 = (s_44_0.value() as u64);
        // D s_44_2: read-var d:i64
        let s_44_2: i64 = fn_state.d;
        // D s_44_3: cast zx s_44_2 -> i
        let s_44_3: i128 = (i128::try_from(s_44_2).unwrap());
        // D s_44_4: call D_set(s_44_3, s_44_1)
        let s_44_4: () = D_set(state, tracer, s_44_3, s_44_1);
        // N s_44_5: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: return
        return;
    }
}
