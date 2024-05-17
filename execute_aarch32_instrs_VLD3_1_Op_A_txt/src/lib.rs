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
use D_set::*;
use MemU_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VLD3_1_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    d2__arg: i128,
    d3__arg: i128,
    ebytes: i64,
    index: i64,
    m: i64,
    n: i64,
    register_index: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_354326: Bits,
        ga_354325: i64,
        d2: i128,
        ga_354313: Bits,
        ga_354324: u64,
        address: u32,
        d: i128,
        ga_354311: u64,
        ga_354318: i64,
        ga_354312: i64,
        d3: i128,
        ga_354319: Bits,
        ga_354317: u64,
        d__arg: i64,
        d2__arg: i128,
        d3__arg: i128,
        ebytes: i64,
        index: i64,
        m: i64,
        n: i64,
        register_index: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        d__arg,
        d2__arg,
        d3__arg,
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
        // D s_1_4: read-var d:i
        let s_1_4: i128 = fn_state.d;
        // D s_1_5: call D_read(s_1_4)
        let s_1_5: u64 = D_read(state, tracer, s_1_4);
        // D s_1_6: write-var ga#354311 <= s_1_5
        fn_state.ga_354311 = s_1_5;
        // C s_1_7: const #8s : i
        let s_1_7: i128 = 8;
        // D s_1_8: read-var ebytes:i64
        let s_1_8: i64 = fn_state.ebytes;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: mul s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) * (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var ga#354312 <= s_1_13
        fn_state.ga_354312 = s_1_13;
        // D s_1_15: read-var address:u32
        let s_1_15: u32 = fn_state.address;
        // D s_1_16: read-var ebytes:i64
        let s_1_16: i64 = fn_state.ebytes;
        // D s_1_17: call MemU_read(s_1_15, s_1_16)
        let s_1_17: Bits = MemU_read(state, tracer, s_1_15, s_1_16);
        // D s_1_18: write-var ga#354313 <= s_1_17
        fn_state.ga_354313 = s_1_17;
        // N s_1_19: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#354311:u64
        let s_2_0: u64 = fn_state.ga_354311;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 64u16);
        // D s_2_2: read-var index:i64
        let s_2_2: i64 = fn_state.index;
        // D s_2_3: cast zx s_2_2 -> i
        let s_2_3: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_4: read-var ga#354312:i64
        let s_2_4: i64 = fn_state.ga_354312;
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: read-var ga#354313:bv
        let s_2_6: Bits = fn_state.ga_354313;
        // D s_2_7: call Elem_set(s_2_1, s_2_3, s_2_5, s_2_6)
        let s_2_7: Bits = Elem_set(state, tracer, s_2_1, s_2_3, s_2_5, s_2_6);
        // D s_2_8: cast reint s_2_7 -> u64
        let s_2_8: u64 = (s_2_7.value() as u64);
        // D s_2_9: read-var d:i
        let s_2_9: i128 = fn_state.d;
        // D s_2_10: call D_set(s_2_9, s_2_8)
        let s_2_10: () = D_set(state, tracer, s_2_9, s_2_8);
        // D s_2_11: read-var d2:i
        let s_2_11: i128 = fn_state.d2;
        // D s_2_12: call D_read(s_2_11)
        let s_2_12: u64 = D_read(state, tracer, s_2_11);
        // D s_2_13: write-var ga#354317 <= s_2_12
        fn_state.ga_354317 = s_2_12;
        // C s_2_14: const #8s : i
        let s_2_14: i128 = 8;
        // D s_2_15: read-var ebytes:i64
        let s_2_15: i64 = fn_state.ebytes;
        // D s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_17: mul s_2_14 s_2_16
        let s_2_17: i128 = ((s_2_14) * (s_2_16));
        // D s_2_18: cast reint s_2_17 -> i64
        let s_2_18: i64 = (s_2_17 as i64);
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (i128::try_from(s_2_18).unwrap());
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: write-var ga#354318 <= s_2_20
        fn_state.ga_354318 = s_2_20;
        // D s_2_22: read-var address:u32
        let s_2_22: u32 = fn_state.address;
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 32u16);
        // D s_2_24: read-var ebytes:i64
        let s_2_24: i64 = fn_state.ebytes;
        // D s_2_25: cast zx s_2_24 -> i
        let s_2_25: i128 = (i128::try_from(s_2_24).unwrap());
        // D s_2_26: cast cvt s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 128);
        // D s_2_27: add s_2_23 s_2_26
        let s_2_27: Bits = (s_2_23 + s_2_26);
        // D s_2_28: cast reint s_2_27 -> u32
        let s_2_28: u32 = (s_2_27.value() as u32);
        // D s_2_29: read-var ebytes:i64
        let s_2_29: i64 = fn_state.ebytes;
        // D s_2_30: call MemU_read(s_2_28, s_2_29)
        let s_2_30: Bits = MemU_read(state, tracer, s_2_28, s_2_29);
        // D s_2_31: write-var ga#354319 <= s_2_30
        fn_state.ga_354319 = s_2_30;
        // N s_2_32: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#354317:u64
        let s_3_0: u64 = fn_state.ga_354317;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 64u16);
        // D s_3_2: read-var index:i64
        let s_3_2: i64 = fn_state.index;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var ga#354318:i64
        let s_3_4: i64 = fn_state.ga_354318;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: read-var ga#354319:bv
        let s_3_6: Bits = fn_state.ga_354319;
        // D s_3_7: call Elem_set(s_3_1, s_3_3, s_3_5, s_3_6)
        let s_3_7: Bits = Elem_set(state, tracer, s_3_1, s_3_3, s_3_5, s_3_6);
        // D s_3_8: cast reint s_3_7 -> u64
        let s_3_8: u64 = (s_3_7.value() as u64);
        // D s_3_9: read-var d2:i
        let s_3_9: i128 = fn_state.d2;
        // D s_3_10: call D_set(s_3_9, s_3_8)
        let s_3_10: () = D_set(state, tracer, s_3_9, s_3_8);
        // D s_3_11: read-var d3:i
        let s_3_11: i128 = fn_state.d3;
        // D s_3_12: call D_read(s_3_11)
        let s_3_12: u64 = D_read(state, tracer, s_3_11);
        // D s_3_13: write-var ga#354324 <= s_3_12
        fn_state.ga_354324 = s_3_12;
        // C s_3_14: const #8s : i
        let s_3_14: i128 = 8;
        // D s_3_15: read-var ebytes:i64
        let s_3_15: i64 = fn_state.ebytes;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: mul s_3_14 s_3_16
        let s_3_17: i128 = ((s_3_14) * (s_3_16));
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: write-var ga#354325 <= s_3_20
        fn_state.ga_354325 = s_3_20;
        // C s_3_22: const #2s : i
        let s_3_22: i128 = 2;
        // D s_3_23: read-var ebytes:i64
        let s_3_23: i64 = fn_state.ebytes;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: mul s_3_22 s_3_24
        let s_3_25: i128 = ((s_3_22) * (s_3_24));
        // D s_3_26: cast reint s_3_25 -> i64
        let s_3_26: i64 = (s_3_25 as i64);
        // D s_3_27: read-var address:u32
        let s_3_27: u32 = fn_state.address;
        // D s_3_28: cast zx s_3_27 -> bv
        let s_3_28: Bits = Bits::new(s_3_27 as u128, 32u16);
        // D s_3_29: cast zx s_3_26 -> i
        let s_3_29: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_30: cast cvt s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 128);
        // D s_3_31: add s_3_28 s_3_30
        let s_3_31: Bits = (s_3_28 + s_3_30);
        // D s_3_32: cast reint s_3_31 -> u32
        let s_3_32: u32 = (s_3_31.value() as u32);
        // D s_3_33: read-var ebytes:i64
        let s_3_33: i64 = fn_state.ebytes;
        // D s_3_34: call MemU_read(s_3_32, s_3_33)
        let s_3_34: Bits = MemU_read(state, tracer, s_3_32, s_3_33);
        // D s_3_35: write-var ga#354326 <= s_3_34
        fn_state.ga_354326 = s_3_34;
        // N s_3_36: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#354324:u64
        let s_4_0: u64 = fn_state.ga_354324;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 64u16);
        // D s_4_2: read-var index:i64
        let s_4_2: i64 = fn_state.index;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var ga#354325:i64
        let s_4_4: i64 = fn_state.ga_354325;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: read-var ga#354326:bv
        let s_4_6: Bits = fn_state.ga_354326;
        // D s_4_7: call Elem_set(s_4_1, s_4_3, s_4_5, s_4_6)
        let s_4_7: Bits = Elem_set(state, tracer, s_4_1, s_4_3, s_4_5, s_4_6);
        // D s_4_8: cast reint s_4_7 -> u64
        let s_4_8: u64 = (s_4_7.value() as u64);
        // D s_4_9: read-var d3:i
        let s_4_9: i128 = fn_state.d3;
        // D s_4_10: call D_set(s_4_9, s_4_8)
        let s_4_10: () = D_set(state, tracer, s_4_9, s_4_8);
        // D s_4_11: read-var wback:u8
        let s_4_11: bool = fn_state.wback;
        // N s_4_12: branch s_4_11 b6 b5
        if s_4_11 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var register_index:u8
        let s_6_0: bool = fn_state.register_index;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var n:i64
        let s_7_0: i64 = fn_state.n;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call R_read(s_7_1)
        let s_7_2: u32 = R_read(state, tracer, s_7_1);
        // C s_7_3: const #3s : i
        let s_7_3: i128 = 3;
        // D s_7_4: read-var ebytes:i64
        let s_7_4: i64 = fn_state.ebytes;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: mul s_7_3 s_7_5
        let s_7_6: i128 = ((s_7_3) * (s_7_5));
        // D s_7_7: cast reint s_7_6 -> i64
        let s_7_7: i64 = (s_7_6 as i64);
        // D s_7_8: cast zx s_7_2 -> bv
        let s_7_8: Bits = Bits::new(s_7_2 as u128, 32u16);
        // D s_7_9: cast zx s_7_7 -> i
        let s_7_9: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_10: cast cvt s_7_9 -> bv
        let s_7_10: Bits = Bits::new(s_7_9 as u128, 128);
        // D s_7_11: add s_7_8 s_7_10
        let s_7_11: Bits = (s_7_8 + s_7_10);
        // D s_7_12: cast reint s_7_11 -> u32
        let s_7_12: u32 = (s_7_11.value() as u32);
        // D s_7_13: read-var n:i64
        let s_7_13: i64 = fn_state.n;
        // D s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_15: call R_set(s_7_14, s_7_12)
        let s_7_15: () = R_set(state, tracer, s_7_14, s_7_12);
        // N s_7_16: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var n:i64
        let s_8_0: i64 = fn_state.n;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call R_read(s_8_1)
        let s_8_2: u32 = R_read(state, tracer, s_8_1);
        // D s_8_3: read-var m:i64
        let s_8_3: i64 = fn_state.m;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: call R_read(s_8_4)
        let s_8_5: u32 = R_read(state, tracer, s_8_4);
        // D s_8_6: cast zx s_8_2 -> bv
        let s_8_6: Bits = Bits::new(s_8_2 as u128, 32u16);
        // D s_8_7: cast zx s_8_5 -> bv
        let s_8_7: Bits = Bits::new(s_8_5 as u128, 32u16);
        // D s_8_8: add s_8_6 s_8_7
        let s_8_8: Bits = (s_8_6 + s_8_7);
        // D s_8_9: cast reint s_8_8 -> u32
        let s_8_9: u32 = (s_8_8.value() as u32);
        // D s_8_10: read-var n:i64
        let s_8_10: i64 = fn_state.n;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: call R_set(s_8_11, s_8_9)
        let s_8_12: () = R_set(state, tracer, s_8_11, s_8_9);
        // N s_8_13: return
        return;
    }
}
