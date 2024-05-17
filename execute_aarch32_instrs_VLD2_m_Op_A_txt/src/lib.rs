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
use Elem_set::*;
use R_read::*;
use CheckAdvSIMDEnabled::*;
use R_set::*;
use AArch32_Abort::*;
use D_set::*;
use IsAligned__1::*;
use CreateAccDescASIMD::*;
use MemU_read::*;
use D_read::*;
use AlignmentFault::*;
use common::*;
pub fn execute_aarch32_instrs_VLD2_m_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d__arg: i64,
    d2__arg: i128,
    ebytes: i64,
    elements: i128,
    m: i64,
    n: i64,
    pairs: i64,
    register_index: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        d2: i128,
        ga_354071: i64,
        ga_354081: i128,
        ga_354079: i64,
        gs_310378: i64,
        ga_354078: u64,
        address: u32,
        ga_354072: Bits,
        d: i128,
        accdesc: ProductType9878976b5bcce9c9,
        ga_354070: u64,
        ga_354073: i128,
        gs_310372: i64,
        ga_354080: Bits,
        alignment: i64,
        d__arg: i64,
        d2__arg: i128,
        ebytes: i64,
        elements: i128,
        m: i64,
        n: i64,
        pairs: i64,
        register_index: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        alignment,
        d__arg,
        d2__arg,
        ebytes,
        elements,
        m,
        n,
        pairs,
        register_index,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var d__arg:i64
        let s_0_0: i64 = fn_state.d__arg;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: write-var d <= s_0_1
        fn_state.d = s_0_1;
        // D s_0_3: read-var d2__arg:i
        let s_0_3: i128 = fn_state.d2__arg;
        // D s_0_4: write-var d2 <= s_0_3
        fn_state.d2 = s_0_3;
        // C s_0_5: const #() : ()
        let s_0_5: () = ();
        // S s_0_6: call CheckAdvSIMDEnabled(s_0_5)
        let s_0_6: () = CheckAdvSIMDEnabled(state, tracer, s_0_5);
        // N s_0_7: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: write-var address <= s_1_2
        fn_state.address = s_1_2;
        // C s_1_4: const #0u : u32
        let s_1_4: u32 = 0;
        // C s_1_5: const #0u : u8
        let s_1_5: bool = false;
        // C s_1_6: const #0u : u8
        let s_1_6: bool = false;
        // S s_1_7: call CreateAccDescASIMD(s_1_4, s_1_5, s_1_6)
        let s_1_7: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_4,
            s_1_5,
            s_1_6,
        );
        // D s_1_8: write-var accdesc <= s_1_7
        fn_state.accdesc = s_1_7;
        // D s_1_9: read-var address:u32
        let s_1_9: u32 = fn_state.address;
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 32u16);
        // D s_1_11: read-var alignment:i64
        let s_1_11: i64 = fn_state.alignment;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call IsAligned__1(s_1_10, s_1_12)
        let s_1_13: bool = IsAligned__1(state, tracer, s_1_10, s_1_12);
        // D s_1_14: not s_1_13
        let s_1_14: bool = !s_1_13;
        // N s_1_15: branch s_1_14 b16 b2
        if s_1_14 {
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
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var pairs:i64
        let s_3_2: i64 = fn_state.pairs;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#310372 <= s_3_5
        fn_state.gs_310372 = s_3_5;
        // D s_3_7: write-var r <= s_3_0
        fn_state.r = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var r:i64
        let s_4_0: i64 = fn_state.r;
        // D s_4_1: read-var gs#310372:i64
        let s_4_1: i64 = fn_state.gs_310372;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b11 b5
        if s_4_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i
        let s_5_2: i128 = fn_state.elements;
        // D s_5_3: sub s_5_2 s_5_1
        let s_5_3: i128 = ((s_5_2) - (s_5_1));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var gs#310378 <= s_5_4
        fn_state.gs_310378 = s_5_4;
        // D s_5_6: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#310378:i64
        let s_6_1: i64 = fn_state.gs_310378;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b10 b7
        if s_6_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r:i64
        let s_7_0: i64 = fn_state.r;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var d:i
        let s_7_2: i128 = fn_state.d;
        // D s_7_3: add s_7_2 s_7_1
        let s_7_3: i128 = (s_7_2 + s_7_1);
        // D s_7_4: write-var ga#354073 <= s_7_3
        fn_state.ga_354073 = s_7_3;
        // D s_7_5: read-var r:i64
        let s_7_5: i64 = fn_state.r;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: read-var d:i
        let s_7_7: i128 = fn_state.d;
        // D s_7_8: add s_7_7 s_7_6
        let s_7_8: i128 = (s_7_7 + s_7_6);
        // D s_7_9: call D_read(s_7_8)
        let s_7_9: u64 = D_read(state, tracer, s_7_8);
        // D s_7_10: write-var ga#354070 <= s_7_9
        fn_state.ga_354070 = s_7_9;
        // C s_7_11: const #8s : i
        let s_7_11: i128 = 8;
        // D s_7_12: read-var ebytes:i64
        let s_7_12: i64 = fn_state.ebytes;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: mul s_7_11 s_7_13
        let s_7_14: i128 = ((s_7_11) * (s_7_13));
        // D s_7_15: cast reint s_7_14 -> i64
        let s_7_15: i64 = (s_7_14 as i64);
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (i128::try_from(s_7_15).unwrap());
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: write-var ga#354071 <= s_7_17
        fn_state.ga_354071 = s_7_17;
        // D s_7_19: read-var address:u32
        let s_7_19: u32 = fn_state.address;
        // D s_7_20: read-var ebytes:i64
        let s_7_20: i64 = fn_state.ebytes;
        // D s_7_21: call MemU_read(s_7_19, s_7_20)
        let s_7_21: Bits = MemU_read(state, tracer, s_7_19, s_7_20);
        // D s_7_22: write-var ga#354072 <= s_7_21
        fn_state.ga_354072 = s_7_21;
        // N s_7_23: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#354070:u64
        let s_8_0: u64 = fn_state.ga_354070;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 64u16);
        // D s_8_2: read-var e:i64
        let s_8_2: i64 = fn_state.e;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var ga#354071:i64
        let s_8_4: i64 = fn_state.ga_354071;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var ga#354072:bv
        let s_8_6: Bits = fn_state.ga_354072;
        // D s_8_7: call Elem_set(s_8_1, s_8_3, s_8_5, s_8_6)
        let s_8_7: Bits = Elem_set(state, tracer, s_8_1, s_8_3, s_8_5, s_8_6);
        // D s_8_8: cast reint s_8_7 -> u64
        let s_8_8: u64 = (s_8_7.value() as u64);
        // D s_8_9: read-var ga#354073:i
        let s_8_9: i128 = fn_state.ga_354073;
        // D s_8_10: call D_set(s_8_9, s_8_8)
        let s_8_10: () = D_set(state, tracer, s_8_9, s_8_8);
        // D s_8_11: read-var r:i64
        let s_8_11: i64 = fn_state.r;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: read-var d2:i
        let s_8_13: i128 = fn_state.d2;
        // D s_8_14: add s_8_13 s_8_12
        let s_8_14: i128 = (s_8_13 + s_8_12);
        // D s_8_15: write-var ga#354081 <= s_8_14
        fn_state.ga_354081 = s_8_14;
        // D s_8_16: read-var r:i64
        let s_8_16: i64 = fn_state.r;
        // D s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_18: read-var d2:i
        let s_8_18: i128 = fn_state.d2;
        // D s_8_19: add s_8_18 s_8_17
        let s_8_19: i128 = (s_8_18 + s_8_17);
        // D s_8_20: call D_read(s_8_19)
        let s_8_20: u64 = D_read(state, tracer, s_8_19);
        // D s_8_21: write-var ga#354078 <= s_8_20
        fn_state.ga_354078 = s_8_20;
        // C s_8_22: const #8s : i
        let s_8_22: i128 = 8;
        // D s_8_23: read-var ebytes:i64
        let s_8_23: i64 = fn_state.ebytes;
        // D s_8_24: cast zx s_8_23 -> i
        let s_8_24: i128 = (i128::try_from(s_8_23).unwrap());
        // D s_8_25: mul s_8_22 s_8_24
        let s_8_25: i128 = ((s_8_22) * (s_8_24));
        // D s_8_26: cast reint s_8_25 -> i64
        let s_8_26: i64 = (s_8_25 as i64);
        // D s_8_27: cast zx s_8_26 -> i
        let s_8_27: i128 = (i128::try_from(s_8_26).unwrap());
        // D s_8_28: cast reint s_8_27 -> i64
        let s_8_28: i64 = (s_8_27 as i64);
        // D s_8_29: write-var ga#354079 <= s_8_28
        fn_state.ga_354079 = s_8_28;
        // D s_8_30: read-var address:u32
        let s_8_30: u32 = fn_state.address;
        // D s_8_31: cast zx s_8_30 -> bv
        let s_8_31: Bits = Bits::new(s_8_30 as u128, 32u16);
        // D s_8_32: read-var ebytes:i64
        let s_8_32: i64 = fn_state.ebytes;
        // D s_8_33: cast zx s_8_32 -> i
        let s_8_33: i128 = (i128::try_from(s_8_32).unwrap());
        // D s_8_34: cast cvt s_8_33 -> bv
        let s_8_34: Bits = Bits::new(s_8_33 as u128, 128);
        // D s_8_35: add s_8_31 s_8_34
        let s_8_35: Bits = (s_8_31 + s_8_34);
        // D s_8_36: cast reint s_8_35 -> u32
        let s_8_36: u32 = (s_8_35.value() as u32);
        // D s_8_37: read-var ebytes:i64
        let s_8_37: i64 = fn_state.ebytes;
        // D s_8_38: call MemU_read(s_8_36, s_8_37)
        let s_8_38: Bits = MemU_read(state, tracer, s_8_36, s_8_37);
        // D s_8_39: write-var ga#354080 <= s_8_38
        fn_state.ga_354080 = s_8_38;
        // N s_8_40: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#354078:u64
        let s_9_0: u64 = fn_state.ga_354078;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 64u16);
        // D s_9_2: read-var e:i64
        let s_9_2: i64 = fn_state.e;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var ga#354079:i64
        let s_9_4: i64 = fn_state.ga_354079;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: read-var ga#354080:bv
        let s_9_6: Bits = fn_state.ga_354080;
        // D s_9_7: call Elem_set(s_9_1, s_9_3, s_9_5, s_9_6)
        let s_9_7: Bits = Elem_set(state, tracer, s_9_1, s_9_3, s_9_5, s_9_6);
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // D s_9_9: read-var ga#354081:i
        let s_9_9: i128 = fn_state.ga_354081;
        // D s_9_10: call D_set(s_9_9, s_9_8)
        let s_9_10: () = D_set(state, tracer, s_9_9, s_9_8);
        // C s_9_11: const #2s : i
        let s_9_11: i128 = 2;
        // D s_9_12: read-var ebytes:i64
        let s_9_12: i64 = fn_state.ebytes;
        // D s_9_13: cast zx s_9_12 -> i
        let s_9_13: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_14: mul s_9_11 s_9_13
        let s_9_14: i128 = ((s_9_11) * (s_9_13));
        // D s_9_15: cast reint s_9_14 -> i64
        let s_9_15: i64 = (s_9_14 as i64);
        // D s_9_16: read-var address:u32
        let s_9_16: u32 = fn_state.address;
        // D s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 32u16);
        // D s_9_18: cast zx s_9_15 -> i
        let s_9_18: i128 = (i128::try_from(s_9_15).unwrap());
        // D s_9_19: cast cvt s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 128);
        // D s_9_20: add s_9_17 s_9_19
        let s_9_20: Bits = (s_9_17 + s_9_19);
        // D s_9_21: cast reint s_9_20 -> u32
        let s_9_21: u32 = (s_9_20.value() as u32);
        // D s_9_22: write-var address <= s_9_21
        fn_state.address = s_9_21;
        // D s_9_23: read-var e:i64
        let s_9_23: i64 = fn_state.e;
        // C s_9_24: const #1s : i64
        let s_9_24: i64 = 1;
        // D s_9_25: add s_9_23 s_9_24
        let s_9_25: i64 = (s_9_23 + s_9_24);
        // D s_9_26: write-var e <= s_9_25
        fn_state.e = s_9_25;
        // N s_9_27: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var r:i64
        let s_10_0: i64 = fn_state.r;
        // C s_10_1: const #1s : i64
        let s_10_1: i64 = 1;
        // D s_10_2: add s_10_0 s_10_1
        let s_10_2: i64 = (s_10_0 + s_10_1);
        // D s_10_3: write-var r <= s_10_2
        fn_state.r = s_10_2;
        // N s_10_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var wback:u8
        let s_11_0: bool = fn_state.wback;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var register_index:u8
        let s_13_0: bool = fn_state.register_index;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var n:i64
        let s_14_0: i64 = fn_state.n;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call R_read(s_14_1)
        let s_14_2: u32 = R_read(state, tracer, s_14_1);
        // C s_14_3: const #16s : i
        let s_14_3: i128 = 16;
        // D s_14_4: read-var pairs:i64
        let s_14_4: i64 = fn_state.pairs;
        // D s_14_5: cast zx s_14_4 -> i
        let s_14_5: i128 = (i128::try_from(s_14_4).unwrap());
        // D s_14_6: mul s_14_3 s_14_5
        let s_14_6: i128 = ((s_14_3) * (s_14_5));
        // D s_14_7: cast reint s_14_6 -> i64
        let s_14_7: i64 = (s_14_6 as i64);
        // D s_14_8: cast zx s_14_2 -> bv
        let s_14_8: Bits = Bits::new(s_14_2 as u128, 32u16);
        // D s_14_9: cast zx s_14_7 -> i
        let s_14_9: i128 = (i128::try_from(s_14_7).unwrap());
        // D s_14_10: cast cvt s_14_9 -> bv
        let s_14_10: Bits = Bits::new(s_14_9 as u128, 128);
        // D s_14_11: add s_14_8 s_14_10
        let s_14_11: Bits = (s_14_8 + s_14_10);
        // D s_14_12: cast reint s_14_11 -> u32
        let s_14_12: u32 = (s_14_11.value() as u32);
        // D s_14_13: read-var n:i64
        let s_14_13: i64 = fn_state.n;
        // D s_14_14: cast zx s_14_13 -> i
        let s_14_14: i128 = (i128::try_from(s_14_13).unwrap());
        // D s_14_15: call R_set(s_14_14, s_14_12)
        let s_14_15: () = R_set(state, tracer, s_14_14, s_14_12);
        // N s_14_16: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call R_read(s_15_1)
        let s_15_2: u32 = R_read(state, tracer, s_15_1);
        // D s_15_3: read-var m:i64
        let s_15_3: i64 = fn_state.m;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: call R_read(s_15_4)
        let s_15_5: u32 = R_read(state, tracer, s_15_4);
        // D s_15_6: cast zx s_15_2 -> bv
        let s_15_6: Bits = Bits::new(s_15_2 as u128, 32u16);
        // D s_15_7: cast zx s_15_5 -> bv
        let s_15_7: Bits = Bits::new(s_15_5 as u128, 32u16);
        // D s_15_8: add s_15_6 s_15_7
        let s_15_8: Bits = (s_15_6 + s_15_7);
        // D s_15_9: cast reint s_15_8 -> u32
        let s_15_9: u32 = (s_15_8.value() as u32);
        // D s_15_10: read-var n:i64
        let s_15_10: i64 = fn_state.n;
        // D s_15_11: cast zx s_15_10 -> i
        let s_15_11: i128 = (i128::try_from(s_15_10).unwrap());
        // D s_15_12: call R_set(s_15_11, s_15_9)
        let s_15_12: () = R_set(state, tracer, s_15_11, s_15_9);
        // N s_15_13: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var accdesc:struct
        let s_16_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_16_1: call AlignmentFault(s_16_0)
        let s_16_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_16_0);
        // D s_16_2: read-var address:u32
        let s_16_2: u32 = fn_state.address;
        // D s_16_3: call AArch32_Abort(s_16_2, s_16_1)
        let s_16_3: () = AArch32_Abort(state, tracer, s_16_2, s_16_1);
        // N s_16_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
