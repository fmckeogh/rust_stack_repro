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
pub fn execute_aarch32_instrs_VLD4_1_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d__arg: i64,
    d2__arg: i128,
    d3__arg: i128,
    d4__arg: i128,
    ebytes: i64,
    index: i64,
    m: i64,
    n: i64,
    register_index: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_354601: u64,
        ga_354597: Bits,
        address: u32,
        ga_354615: u64,
        ga_354609: i64,
        ga_354610: Bits,
        ga_354617: Bits,
        ga_354595: u64,
        ga_354603: Bits,
        ga_354608: u64,
        d2: i128,
        d: i128,
        ga_354602: i64,
        accdesc: ProductType9878976b5bcce9c9,
        d4: i128,
        d3: i128,
        ga_354616: i64,
        ga_354596: i64,
        alignment: i64,
        d__arg: i64,
        d2__arg: i128,
        d3__arg: i128,
        d4__arg: i128,
        ebytes: i64,
        index: i64,
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
        d4__arg,
        ebytes,
        index,
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
        // D s_0_7: read-var d4__arg:i
        let s_0_7: i128 = fn_state.d4__arg;
        // D s_0_8: write-var d4 <= s_0_7
        fn_state.d4 = s_0_7;
        // C s_0_9: const #() : ()
        let s_0_9: () = ();
        // S s_0_10: call CheckAdvSIMDEnabled(s_0_9)
        let s_0_10: () = CheckAdvSIMDEnabled(state, tracer, s_0_9);
        // N s_0_11: jump b1
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
        // N s_1_15: branch s_1_14 b12 b2
        if s_1_14 {
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
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i
        let s_3_0: i128 = fn_state.d;
        // D s_3_1: call D_read(s_3_0)
        let s_3_1: u64 = D_read(state, tracer, s_3_0);
        // D s_3_2: write-var ga#354595 <= s_3_1
        fn_state.ga_354595 = s_3_1;
        // C s_3_3: const #8s : i
        let s_3_3: i128 = 8;
        // D s_3_4: read-var ebytes:i64
        let s_3_4: i64 = fn_state.ebytes;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: mul s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) * (s_3_5));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var ga#354596 <= s_3_9
        fn_state.ga_354596 = s_3_9;
        // D s_3_11: read-var address:u32
        let s_3_11: u32 = fn_state.address;
        // D s_3_12: read-var ebytes:i64
        let s_3_12: i64 = fn_state.ebytes;
        // D s_3_13: call MemU_read(s_3_11, s_3_12)
        let s_3_13: Bits = MemU_read(state, tracer, s_3_11, s_3_12);
        // D s_3_14: write-var ga#354597 <= s_3_13
        fn_state.ga_354597 = s_3_13;
        // N s_3_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#354595:u64
        let s_4_0: u64 = fn_state.ga_354595;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 64u16);
        // D s_4_2: read-var index:i64
        let s_4_2: i64 = fn_state.index;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var ga#354596:i64
        let s_4_4: i64 = fn_state.ga_354596;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: read-var ga#354597:bv
        let s_4_6: Bits = fn_state.ga_354597;
        // D s_4_7: call Elem_set(s_4_1, s_4_3, s_4_5, s_4_6)
        let s_4_7: Bits = Elem_set(state, tracer, s_4_1, s_4_3, s_4_5, s_4_6);
        // D s_4_8: cast reint s_4_7 -> u64
        let s_4_8: u64 = (s_4_7.value() as u64);
        // D s_4_9: read-var d:i
        let s_4_9: i128 = fn_state.d;
        // D s_4_10: call D_set(s_4_9, s_4_8)
        let s_4_10: () = D_set(state, tracer, s_4_9, s_4_8);
        // D s_4_11: read-var d2:i
        let s_4_11: i128 = fn_state.d2;
        // D s_4_12: call D_read(s_4_11)
        let s_4_12: u64 = D_read(state, tracer, s_4_11);
        // D s_4_13: write-var ga#354601 <= s_4_12
        fn_state.ga_354601 = s_4_12;
        // C s_4_14: const #8s : i
        let s_4_14: i128 = 8;
        // D s_4_15: read-var ebytes:i64
        let s_4_15: i64 = fn_state.ebytes;
        // D s_4_16: cast zx s_4_15 -> i
        let s_4_16: i128 = (i128::try_from(s_4_15).unwrap());
        // D s_4_17: mul s_4_14 s_4_16
        let s_4_17: i128 = ((s_4_14) * (s_4_16));
        // D s_4_18: cast reint s_4_17 -> i64
        let s_4_18: i64 = (s_4_17 as i64);
        // D s_4_19: cast zx s_4_18 -> i
        let s_4_19: i128 = (i128::try_from(s_4_18).unwrap());
        // D s_4_20: cast reint s_4_19 -> i64
        let s_4_20: i64 = (s_4_19 as i64);
        // D s_4_21: write-var ga#354602 <= s_4_20
        fn_state.ga_354602 = s_4_20;
        // D s_4_22: read-var address:u32
        let s_4_22: u32 = fn_state.address;
        // D s_4_23: cast zx s_4_22 -> bv
        let s_4_23: Bits = Bits::new(s_4_22 as u128, 32u16);
        // D s_4_24: read-var ebytes:i64
        let s_4_24: i64 = fn_state.ebytes;
        // D s_4_25: cast zx s_4_24 -> i
        let s_4_25: i128 = (i128::try_from(s_4_24).unwrap());
        // D s_4_26: cast cvt s_4_25 -> bv
        let s_4_26: Bits = Bits::new(s_4_25 as u128, 128);
        // D s_4_27: add s_4_23 s_4_26
        let s_4_27: Bits = (s_4_23 + s_4_26);
        // D s_4_28: cast reint s_4_27 -> u32
        let s_4_28: u32 = (s_4_27.value() as u32);
        // D s_4_29: read-var ebytes:i64
        let s_4_29: i64 = fn_state.ebytes;
        // D s_4_30: call MemU_read(s_4_28, s_4_29)
        let s_4_30: Bits = MemU_read(state, tracer, s_4_28, s_4_29);
        // D s_4_31: write-var ga#354603 <= s_4_30
        fn_state.ga_354603 = s_4_30;
        // N s_4_32: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#354601:u64
        let s_5_0: u64 = fn_state.ga_354601;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 64u16);
        // D s_5_2: read-var index:i64
        let s_5_2: i64 = fn_state.index;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var ga#354602:i64
        let s_5_4: i64 = fn_state.ga_354602;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var ga#354603:bv
        let s_5_6: Bits = fn_state.ga_354603;
        // D s_5_7: call Elem_set(s_5_1, s_5_3, s_5_5, s_5_6)
        let s_5_7: Bits = Elem_set(state, tracer, s_5_1, s_5_3, s_5_5, s_5_6);
        // D s_5_8: cast reint s_5_7 -> u64
        let s_5_8: u64 = (s_5_7.value() as u64);
        // D s_5_9: read-var d2:i
        let s_5_9: i128 = fn_state.d2;
        // D s_5_10: call D_set(s_5_9, s_5_8)
        let s_5_10: () = D_set(state, tracer, s_5_9, s_5_8);
        // D s_5_11: read-var d3:i
        let s_5_11: i128 = fn_state.d3;
        // D s_5_12: call D_read(s_5_11)
        let s_5_12: u64 = D_read(state, tracer, s_5_11);
        // D s_5_13: write-var ga#354608 <= s_5_12
        fn_state.ga_354608 = s_5_12;
        // C s_5_14: const #8s : i
        let s_5_14: i128 = 8;
        // D s_5_15: read-var ebytes:i64
        let s_5_15: i64 = fn_state.ebytes;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: mul s_5_14 s_5_16
        let s_5_17: i128 = ((s_5_14) * (s_5_16));
        // D s_5_18: cast reint s_5_17 -> i64
        let s_5_18: i64 = (s_5_17 as i64);
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: cast reint s_5_19 -> i64
        let s_5_20: i64 = (s_5_19 as i64);
        // D s_5_21: write-var ga#354609 <= s_5_20
        fn_state.ga_354609 = s_5_20;
        // C s_5_22: const #2s : i
        let s_5_22: i128 = 2;
        // D s_5_23: read-var ebytes:i64
        let s_5_23: i64 = fn_state.ebytes;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: mul s_5_22 s_5_24
        let s_5_25: i128 = ((s_5_22) * (s_5_24));
        // D s_5_26: cast reint s_5_25 -> i64
        let s_5_26: i64 = (s_5_25 as i64);
        // D s_5_27: read-var address:u32
        let s_5_27: u32 = fn_state.address;
        // D s_5_28: cast zx s_5_27 -> bv
        let s_5_28: Bits = Bits::new(s_5_27 as u128, 32u16);
        // D s_5_29: cast zx s_5_26 -> i
        let s_5_29: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_30: cast cvt s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 128);
        // D s_5_31: add s_5_28 s_5_30
        let s_5_31: Bits = (s_5_28 + s_5_30);
        // D s_5_32: cast reint s_5_31 -> u32
        let s_5_32: u32 = (s_5_31.value() as u32);
        // D s_5_33: read-var ebytes:i64
        let s_5_33: i64 = fn_state.ebytes;
        // D s_5_34: call MemU_read(s_5_32, s_5_33)
        let s_5_34: Bits = MemU_read(state, tracer, s_5_32, s_5_33);
        // D s_5_35: write-var ga#354610 <= s_5_34
        fn_state.ga_354610 = s_5_34;
        // N s_5_36: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#354608:u64
        let s_6_0: u64 = fn_state.ga_354608;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 64u16);
        // D s_6_2: read-var index:i64
        let s_6_2: i64 = fn_state.index;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-var ga#354609:i64
        let s_6_4: i64 = fn_state.ga_354609;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var ga#354610:bv
        let s_6_6: Bits = fn_state.ga_354610;
        // D s_6_7: call Elem_set(s_6_1, s_6_3, s_6_5, s_6_6)
        let s_6_7: Bits = Elem_set(state, tracer, s_6_1, s_6_3, s_6_5, s_6_6);
        // D s_6_8: cast reint s_6_7 -> u64
        let s_6_8: u64 = (s_6_7.value() as u64);
        // D s_6_9: read-var d3:i
        let s_6_9: i128 = fn_state.d3;
        // D s_6_10: call D_set(s_6_9, s_6_8)
        let s_6_10: () = D_set(state, tracer, s_6_9, s_6_8);
        // D s_6_11: read-var d4:i
        let s_6_11: i128 = fn_state.d4;
        // D s_6_12: call D_read(s_6_11)
        let s_6_12: u64 = D_read(state, tracer, s_6_11);
        // D s_6_13: write-var ga#354615 <= s_6_12
        fn_state.ga_354615 = s_6_12;
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
        // D s_6_21: write-var ga#354616 <= s_6_20
        fn_state.ga_354616 = s_6_20;
        // C s_6_22: const #3s : i
        let s_6_22: i128 = 3;
        // D s_6_23: read-var ebytes:i64
        let s_6_23: i64 = fn_state.ebytes;
        // D s_6_24: cast zx s_6_23 -> i
        let s_6_24: i128 = (i128::try_from(s_6_23).unwrap());
        // D s_6_25: mul s_6_22 s_6_24
        let s_6_25: i128 = ((s_6_22) * (s_6_24));
        // D s_6_26: cast reint s_6_25 -> i64
        let s_6_26: i64 = (s_6_25 as i64);
        // D s_6_27: read-var address:u32
        let s_6_27: u32 = fn_state.address;
        // D s_6_28: cast zx s_6_27 -> bv
        let s_6_28: Bits = Bits::new(s_6_27 as u128, 32u16);
        // D s_6_29: cast zx s_6_26 -> i
        let s_6_29: i128 = (i128::try_from(s_6_26).unwrap());
        // D s_6_30: cast cvt s_6_29 -> bv
        let s_6_30: Bits = Bits::new(s_6_29 as u128, 128);
        // D s_6_31: add s_6_28 s_6_30
        let s_6_31: Bits = (s_6_28 + s_6_30);
        // D s_6_32: cast reint s_6_31 -> u32
        let s_6_32: u32 = (s_6_31.value() as u32);
        // D s_6_33: read-var ebytes:i64
        let s_6_33: i64 = fn_state.ebytes;
        // D s_6_34: call MemU_read(s_6_32, s_6_33)
        let s_6_34: Bits = MemU_read(state, tracer, s_6_32, s_6_33);
        // D s_6_35: write-var ga#354617 <= s_6_34
        fn_state.ga_354617 = s_6_34;
        // N s_6_36: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#354615:u64
        let s_7_0: u64 = fn_state.ga_354615;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var index:i64
        let s_7_2: i64 = fn_state.index;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var ga#354616:i64
        let s_7_4: i64 = fn_state.ga_354616;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var ga#354617:bv
        let s_7_6: Bits = fn_state.ga_354617;
        // D s_7_7: call Elem_set(s_7_1, s_7_3, s_7_5, s_7_6)
        let s_7_7: Bits = Elem_set(state, tracer, s_7_1, s_7_3, s_7_5, s_7_6);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // D s_7_9: read-var d4:i
        let s_7_9: i128 = fn_state.d4;
        // D s_7_10: call D_set(s_7_9, s_7_8)
        let s_7_10: () = D_set(state, tracer, s_7_9, s_7_8);
        // D s_7_11: read-var wback:u8
        let s_7_11: bool = fn_state.wback;
        // N s_7_12: branch s_7_11 b9 b8
        if s_7_11 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var register_index:u8
        let s_9_0: bool = fn_state.register_index;
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
        // D s_10_0: read-var n:i64
        let s_10_0: i64 = fn_state.n;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call R_read(s_10_1)
        let s_10_2: u32 = R_read(state, tracer, s_10_1);
        // C s_10_3: const #4s : i
        let s_10_3: i128 = 4;
        // D s_10_4: read-var ebytes:i64
        let s_10_4: i64 = fn_state.ebytes;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: mul s_10_3 s_10_5
        let s_10_6: i128 = ((s_10_3) * (s_10_5));
        // D s_10_7: cast reint s_10_6 -> i64
        let s_10_7: i64 = (s_10_6 as i64);
        // D s_10_8: cast zx s_10_2 -> bv
        let s_10_8: Bits = Bits::new(s_10_2 as u128, 32u16);
        // D s_10_9: cast zx s_10_7 -> i
        let s_10_9: i128 = (i128::try_from(s_10_7).unwrap());
        // D s_10_10: cast cvt s_10_9 -> bv
        let s_10_10: Bits = Bits::new(s_10_9 as u128, 128);
        // D s_10_11: add s_10_8 s_10_10
        let s_10_11: Bits = (s_10_8 + s_10_10);
        // D s_10_12: cast reint s_10_11 -> u32
        let s_10_12: u32 = (s_10_11.value() as u32);
        // D s_10_13: read-var n:i64
        let s_10_13: i64 = fn_state.n;
        // D s_10_14: cast zx s_10_13 -> i
        let s_10_14: i128 = (i128::try_from(s_10_13).unwrap());
        // D s_10_15: call R_set(s_10_14, s_10_12)
        let s_10_15: () = R_set(state, tracer, s_10_14, s_10_12);
        // N s_10_16: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var n:i64
        let s_11_0: i64 = fn_state.n;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call R_read(s_11_1)
        let s_11_2: u32 = R_read(state, tracer, s_11_1);
        // D s_11_3: read-var m:i64
        let s_11_3: i64 = fn_state.m;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: call R_read(s_11_4)
        let s_11_5: u32 = R_read(state, tracer, s_11_4);
        // D s_11_6: cast zx s_11_2 -> bv
        let s_11_6: Bits = Bits::new(s_11_2 as u128, 32u16);
        // D s_11_7: cast zx s_11_5 -> bv
        let s_11_7: Bits = Bits::new(s_11_5 as u128, 32u16);
        // D s_11_8: add s_11_6 s_11_7
        let s_11_8: Bits = (s_11_6 + s_11_7);
        // D s_11_9: cast reint s_11_8 -> u32
        let s_11_9: u32 = (s_11_8.value() as u32);
        // D s_11_10: read-var n:i64
        let s_11_10: i64 = fn_state.n;
        // D s_11_11: cast zx s_11_10 -> i
        let s_11_11: i128 = (i128::try_from(s_11_10).unwrap());
        // D s_11_12: call R_set(s_11_11, s_11_9)
        let s_11_12: () = R_set(state, tracer, s_11_11, s_11_9);
        // N s_11_13: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var accdesc:struct
        let s_12_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_1: call AlignmentFault(s_12_0)
        let s_12_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_12_0);
        // D s_12_2: read-var address:u32
        let s_12_2: u32 = fn_state.address;
        // D s_12_3: call AArch32_Abort(s_12_2, s_12_1)
        let s_12_3: () = AArch32_Abort(state, tracer, s_12_2, s_12_1);
        // N s_12_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
