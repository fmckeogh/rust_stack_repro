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
pub fn execute_aarch32_instrs_VLD3_m_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d__arg: i64,
    d2__arg: i128,
    d3__arg: i128,
    ebytes: i64,
    elements: i128,
    m: i64,
    n: i64,
    register_index: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_354520: u64,
        ga_354533: u64,
        ga_354528: Bits,
        e: i64,
        d2: i128,
        ga_354527: i64,
        ga_354522: Bits,
        gs_310961: i64,
        address: u32,
        ga_354535: Bits,
        d: i128,
        accdesc: ProductType9878976b5bcce9c9,
        ga_354526: u64,
        ga_354521: i64,
        ga_354534: i64,
        d3: i128,
        alignment: i64,
        d__arg: i64,
        d2__arg: i128,
        d3__arg: i128,
        ebytes: i64,
        elements: i128,
        m: i64,
        n: i64,
        register_index: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        alignment,
        d__arg,
        d2__arg,
        d3__arg,
        ebytes,
        elements,
        m,
        n,
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
        // D s_0_5: read-var d3__arg:i
        let s_0_5: i128 = fn_state.d3__arg;
        // D s_0_6: write-var d3 <= s_0_5
        fn_state.d3 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
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
        // N s_1_15: branch s_1_14 b14 b2
        if s_1_14 {
            return block_14(state, tracer, fn_state);
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
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#310961 <= s_3_4
        fn_state.gs_310961 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#310961:i64
        let s_4_1: i64 = fn_state.gs_310961;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var d:i
        let s_5_0: i128 = fn_state.d;
        // D s_5_1: call D_read(s_5_0)
        let s_5_1: u64 = D_read(state, tracer, s_5_0);
        // D s_5_2: write-var ga#354520 <= s_5_1
        fn_state.ga_354520 = s_5_1;
        // C s_5_3: const #8s : i
        let s_5_3: i128 = 8;
        // D s_5_4: read-var ebytes:i64
        let s_5_4: i64 = fn_state.ebytes;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: mul s_5_3 s_5_5
        let s_5_6: i128 = ((s_5_3) * (s_5_5));
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: write-var ga#354521 <= s_5_9
        fn_state.ga_354521 = s_5_9;
        // D s_5_11: read-var address:u32
        let s_5_11: u32 = fn_state.address;
        // D s_5_12: read-var ebytes:i64
        let s_5_12: i64 = fn_state.ebytes;
        // D s_5_13: call MemU_read(s_5_11, s_5_12)
        let s_5_13: Bits = MemU_read(state, tracer, s_5_11, s_5_12);
        // D s_5_14: write-var ga#354522 <= s_5_13
        fn_state.ga_354522 = s_5_13;
        // N s_5_15: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#354520:u64
        let s_6_0: u64 = fn_state.ga_354520;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 64u16);
        // D s_6_2: read-var e:i64
        let s_6_2: i64 = fn_state.e;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var ga#354521:i64
        let s_6_4: i64 = fn_state.ga_354521;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var ga#354522:bv
        let s_6_6: Bits = fn_state.ga_354522;
        // D s_6_7: call Elem_set(s_6_1, s_6_3, s_6_5, s_6_6)
        let s_6_7: Bits = Elem_set(state, tracer, s_6_1, s_6_3, s_6_5, s_6_6);
        // D s_6_8: cast reint s_6_7 -> u64
        let s_6_8: u64 = (s_6_7.value() as u64);
        // D s_6_9: read-var d:i
        let s_6_9: i128 = fn_state.d;
        // D s_6_10: call D_set(s_6_9, s_6_8)
        let s_6_10: () = D_set(state, tracer, s_6_9, s_6_8);
        // D s_6_11: read-var d2:i
        let s_6_11: i128 = fn_state.d2;
        // D s_6_12: call D_read(s_6_11)
        let s_6_12: u64 = D_read(state, tracer, s_6_11);
        // D s_6_13: write-var ga#354526 <= s_6_12
        fn_state.ga_354526 = s_6_12;
        // C s_6_14: const #8s : i
        let s_6_14: i128 = 8;
        // D s_6_15: read-var ebytes:i64
        let s_6_15: i64 = fn_state.ebytes;
        // D s_6_16: cast zx s_6_15 -> i
        let s_6_16: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_17: mul s_6_14 s_6_16
        let s_6_17: i128 = ((s_6_14) * (s_6_16));
        // D s_6_18: cast reint s_6_17 -> i64
        let s_6_18: i64 = (s_6_17 as i64);
        // D s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // D s_6_20: cast reint s_6_19 -> i64
        let s_6_20: i64 = (s_6_19 as i64);
        // D s_6_21: write-var ga#354527 <= s_6_20
        fn_state.ga_354527 = s_6_20;
        // D s_6_22: read-var address:u32
        let s_6_22: u32 = fn_state.address;
        // D s_6_23: cast zx s_6_22 -> bv
        let s_6_23: Bits = Bits::new(s_6_22 as u128, 32u16);
        // D s_6_24: read-var ebytes:i64
        let s_6_24: i64 = fn_state.ebytes;
        // D s_6_25: cast zx s_6_24 -> i
        let s_6_25: i128 = (i128::try_from(s_6_24).unwrap());
        // D s_6_26: cast cvt s_6_25 -> bv
        let s_6_26: Bits = Bits::new(s_6_25 as u128, 128);
        // D s_6_27: add s_6_23 s_6_26
        let s_6_27: Bits = (s_6_23 + s_6_26);
        // D s_6_28: cast reint s_6_27 -> u32
        let s_6_28: u32 = (s_6_27.value() as u32);
        // D s_6_29: read-var ebytes:i64
        let s_6_29: i64 = fn_state.ebytes;
        // D s_6_30: call MemU_read(s_6_28, s_6_29)
        let s_6_30: Bits = MemU_read(state, tracer, s_6_28, s_6_29);
        // D s_6_31: write-var ga#354528 <= s_6_30
        fn_state.ga_354528 = s_6_30;
        // N s_6_32: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#354526:u64
        let s_7_0: u64 = fn_state.ga_354526;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var e:i64
        let s_7_2: i64 = fn_state.e;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var ga#354527:i64
        let s_7_4: i64 = fn_state.ga_354527;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var ga#354528:bv
        let s_7_6: Bits = fn_state.ga_354528;
        // D s_7_7: call Elem_set(s_7_1, s_7_3, s_7_5, s_7_6)
        let s_7_7: Bits = Elem_set(state, tracer, s_7_1, s_7_3, s_7_5, s_7_6);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // D s_7_9: read-var d2:i
        let s_7_9: i128 = fn_state.d2;
        // D s_7_10: call D_set(s_7_9, s_7_8)
        let s_7_10: () = D_set(state, tracer, s_7_9, s_7_8);
        // D s_7_11: read-var d3:i
        let s_7_11: i128 = fn_state.d3;
        // D s_7_12: call D_read(s_7_11)
        let s_7_12: u64 = D_read(state, tracer, s_7_11);
        // D s_7_13: write-var ga#354533 <= s_7_12
        fn_state.ga_354533 = s_7_12;
        // C s_7_14: const #8s : i
        let s_7_14: i128 = 8;
        // D s_7_15: read-var ebytes:i64
        let s_7_15: i64 = fn_state.ebytes;
        // D s_7_16: cast zx s_7_15 -> i
        let s_7_16: i128 = (i128::try_from(s_7_15).unwrap());
        // D s_7_17: mul s_7_14 s_7_16
        let s_7_17: i128 = ((s_7_14) * (s_7_16));
        // D s_7_18: cast reint s_7_17 -> i64
        let s_7_18: i64 = (s_7_17 as i64);
        // D s_7_19: cast zx s_7_18 -> i
        let s_7_19: i128 = (i128::try_from(s_7_18).unwrap());
        // D s_7_20: cast reint s_7_19 -> i64
        let s_7_20: i64 = (s_7_19 as i64);
        // D s_7_21: write-var ga#354534 <= s_7_20
        fn_state.ga_354534 = s_7_20;
        // C s_7_22: const #2s : i
        let s_7_22: i128 = 2;
        // D s_7_23: read-var ebytes:i64
        let s_7_23: i64 = fn_state.ebytes;
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: mul s_7_22 s_7_24
        let s_7_25: i128 = ((s_7_22) * (s_7_24));
        // D s_7_26: cast reint s_7_25 -> i64
        let s_7_26: i64 = (s_7_25 as i64);
        // D s_7_27: read-var address:u32
        let s_7_27: u32 = fn_state.address;
        // D s_7_28: cast zx s_7_27 -> bv
        let s_7_28: Bits = Bits::new(s_7_27 as u128, 32u16);
        // D s_7_29: cast zx s_7_26 -> i
        let s_7_29: i128 = (i128::try_from(s_7_26).unwrap());
        // D s_7_30: cast cvt s_7_29 -> bv
        let s_7_30: Bits = Bits::new(s_7_29 as u128, 128);
        // D s_7_31: add s_7_28 s_7_30
        let s_7_31: Bits = (s_7_28 + s_7_30);
        // D s_7_32: cast reint s_7_31 -> u32
        let s_7_32: u32 = (s_7_31.value() as u32);
        // D s_7_33: read-var ebytes:i64
        let s_7_33: i64 = fn_state.ebytes;
        // D s_7_34: call MemU_read(s_7_32, s_7_33)
        let s_7_34: Bits = MemU_read(state, tracer, s_7_32, s_7_33);
        // D s_7_35: write-var ga#354535 <= s_7_34
        fn_state.ga_354535 = s_7_34;
        // N s_7_36: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#354533:u64
        let s_8_0: u64 = fn_state.ga_354533;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 64u16);
        // D s_8_2: read-var e:i64
        let s_8_2: i64 = fn_state.e;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var ga#354534:i64
        let s_8_4: i64 = fn_state.ga_354534;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var ga#354535:bv
        let s_8_6: Bits = fn_state.ga_354535;
        // D s_8_7: call Elem_set(s_8_1, s_8_3, s_8_5, s_8_6)
        let s_8_7: Bits = Elem_set(state, tracer, s_8_1, s_8_3, s_8_5, s_8_6);
        // D s_8_8: cast reint s_8_7 -> u64
        let s_8_8: u64 = (s_8_7.value() as u64);
        // D s_8_9: read-var d3:i
        let s_8_9: i128 = fn_state.d3;
        // D s_8_10: call D_set(s_8_9, s_8_8)
        let s_8_10: () = D_set(state, tracer, s_8_9, s_8_8);
        // C s_8_11: const #3s : i
        let s_8_11: i128 = 3;
        // D s_8_12: read-var ebytes:i64
        let s_8_12: i64 = fn_state.ebytes;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: mul s_8_11 s_8_13
        let s_8_14: i128 = ((s_8_11) * (s_8_13));
        // D s_8_15: cast reint s_8_14 -> i64
        let s_8_15: i64 = (s_8_14 as i64);
        // D s_8_16: read-var address:u32
        let s_8_16: u32 = fn_state.address;
        // D s_8_17: cast zx s_8_16 -> bv
        let s_8_17: Bits = Bits::new(s_8_16 as u128, 32u16);
        // D s_8_18: cast zx s_8_15 -> i
        let s_8_18: i128 = (i128::try_from(s_8_15).unwrap());
        // D s_8_19: cast cvt s_8_18 -> bv
        let s_8_19: Bits = Bits::new(s_8_18 as u128, 128);
        // D s_8_20: add s_8_17 s_8_19
        let s_8_20: Bits = (s_8_17 + s_8_19);
        // D s_8_21: cast reint s_8_20 -> u32
        let s_8_21: u32 = (s_8_20.value() as u32);
        // D s_8_22: write-var address <= s_8_21
        fn_state.address = s_8_21;
        // D s_8_23: read-var e:i64
        let s_8_23: i64 = fn_state.e;
        // C s_8_24: const #1s : i64
        let s_8_24: i64 = 1;
        // D s_8_25: add s_8_23 s_8_24
        let s_8_25: i64 = (s_8_23 + s_8_24);
        // D s_8_26: write-var e <= s_8_25
        fn_state.e = s_8_25;
        // N s_8_27: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var wback:u8
        let s_9_0: bool = fn_state.wback;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var register_index:u8
        let s_11_0: bool = fn_state.register_index;
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
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call R_read(s_12_1)
        let s_12_2: u32 = R_read(state, tracer, s_12_1);
        // C s_12_3: const #24s : i
        let s_12_3: i128 = 24;
        // D s_12_4: cast zx s_12_2 -> bv
        let s_12_4: Bits = Bits::new(s_12_2 as u128, 32u16);
        // C s_12_5: cast cvt s_12_3 -> bv
        let s_12_5: Bits = Bits::new(s_12_3 as u128, 128);
        // D s_12_6: add s_12_4 s_12_5
        let s_12_6: Bits = (s_12_4 + s_12_5);
        // D s_12_7: cast reint s_12_6 -> u32
        let s_12_7: u32 = (s_12_6.value() as u32);
        // D s_12_8: read-var n:i64
        let s_12_8: i64 = fn_state.n;
        // D s_12_9: cast zx s_12_8 -> i
        let s_12_9: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_10: call R_set(s_12_9, s_12_7)
        let s_12_10: () = R_set(state, tracer, s_12_9, s_12_7);
        // N s_12_11: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call R_read(s_13_1)
        let s_13_2: u32 = R_read(state, tracer, s_13_1);
        // D s_13_3: read-var m:i64
        let s_13_3: i64 = fn_state.m;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: call R_read(s_13_4)
        let s_13_5: u32 = R_read(state, tracer, s_13_4);
        // D s_13_6: cast zx s_13_2 -> bv
        let s_13_6: Bits = Bits::new(s_13_2 as u128, 32u16);
        // D s_13_7: cast zx s_13_5 -> bv
        let s_13_7: Bits = Bits::new(s_13_5 as u128, 32u16);
        // D s_13_8: add s_13_6 s_13_7
        let s_13_8: Bits = (s_13_6 + s_13_7);
        // D s_13_9: cast reint s_13_8 -> u32
        let s_13_9: u32 = (s_13_8.value() as u32);
        // D s_13_10: read-var n:i64
        let s_13_10: i64 = fn_state.n;
        // D s_13_11: cast zx s_13_10 -> i
        let s_13_11: i128 = (i128::try_from(s_13_10).unwrap());
        // D s_13_12: call R_set(s_13_11, s_13_9)
        let s_13_12: () = R_set(state, tracer, s_13_11, s_13_9);
        // N s_13_13: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var accdesc:struct
        let s_14_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_14_1: call AlignmentFault(s_14_0)
        let s_14_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_14_0);
        // D s_14_2: read-var address:u32
        let s_14_2: u32 = fn_state.address;
        // D s_14_3: call AArch32_Abort(s_14_2, s_14_1)
        let s_14_3: () = AArch32_Abort(state, tracer, s_14_2, s_14_1);
        // N s_14_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
