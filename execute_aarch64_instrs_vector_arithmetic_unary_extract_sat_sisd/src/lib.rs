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
use Vpart_set::*;
use asl_Int::*;
use SatQ::*;
use V_read::*;
use Elem_read::*;
use Elem_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_extract_sat_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
    part: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        gs_172401: i64,
        ga_268694: ProductTypef506aa96a892fe52,
        result: Bits,
        esizeshadow_1971: i64,
        datasizeshadow_1972: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
        part: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
        part,
        is_unsigned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#1971 <= s_0_2
        fn_state.esizeshadow_1971 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1972 <= s_0_6
        fn_state.datasizeshadow_1972 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var datasizeshadow#1972:i64
        let s_1_1: i64 = fn_state.datasizeshadow_1972;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: mul s_1_0 s_1_2
        let s_1_3: i128 = ((s_1_0) * (s_1_2));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: write-var operand <= s_1_9
        fn_state.operand = s_1_9;
        // C s_1_11: const #0s : i64
        let s_1_11: i64 = 0;
        // C s_1_12: const #1s : i
        let s_1_12: i128 = 1;
        // D s_1_13: read-var elements:i
        let s_1_13: i128 = fn_state.elements;
        // D s_1_14: sub s_1_13 s_1_12
        let s_1_14: i128 = ((s_1_13) - (s_1_12));
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: write-var gs#172401 <= s_1_15
        fn_state.gs_172401 = s_1_15;
        // D s_1_17: write-var e <= s_1_11
        fn_state.e = s_1_11;
        // N s_1_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#172401:i64
        let s_2_1: i64 = fn_state.gs_172401;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
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
        // C s_3_0: const #2s : i
        let s_3_0: i128 = 2;
        // D s_3_1: read-var esizeshadow#1971:i64
        let s_3_1: i64 = fn_state.esizeshadow_1971;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: cast reint s_3_5 -> i64
        let s_3_6: i64 = (s_3_5 as i64);
        // D s_3_7: read-var e:i64
        let s_3_7: i64 = fn_state.e;
        // D s_3_8: cast zx s_3_7 -> i
        let s_3_8: i128 = (i128::try_from(s_3_7).unwrap());
        // D s_3_9: cast zx s_3_6 -> i
        let s_3_9: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_10: read-var operand:bv
        let s_3_10: Bits = fn_state.operand;
        // D s_3_11: call Elem_read(s_3_10, s_3_8, s_3_9)
        let s_3_11: Bits = Elem_read(state, tracer, s_3_10, s_3_8, s_3_9);
        // D s_3_12: read-var is_unsigned:u8
        let s_3_12: bool = fn_state.is_unsigned;
        // D s_3_13: call asl_Int(s_3_11, s_3_12)
        let s_3_13: i128 = asl_Int(state, tracer, s_3_11, s_3_12);
        // D s_3_14: read-var esizeshadow#1971:i64
        let s_3_14: i64 = fn_state.esizeshadow_1971;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: cast reint s_3_15 -> i64
        let s_3_16: i64 = (s_3_15 as i64);
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: read-var is_unsigned:u8
        let s_3_18: bool = fn_state.is_unsigned;
        // D s_3_19: call SatQ(s_3_13, s_3_17, s_3_18)
        let s_3_19: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_3_13,
            s_3_17,
            s_3_18,
        );
        // D s_3_20: write-var ga#268694 <= s_3_19
        fn_state.ga_268694 = s_3_19;
        // D s_3_21: read-var ga#268694.0:struct
        let s_3_21: Bits = fn_state.ga_268694._0;
        // D s_3_22: read-var ga#268694.1:struct
        let s_3_22: bool = fn_state.ga_268694._1;
        // D s_3_23: read-var esizeshadow#1971:i64
        let s_3_23: i64 = fn_state.esizeshadow_1971;
        // D s_3_24: cast zx s_3_23 -> i
        let s_3_24: i128 = (i128::try_from(s_3_23).unwrap());
        // D s_3_25: cast reint s_3_24 -> i64
        let s_3_25: i64 = (s_3_24 as i64);
        // D s_3_26: read-var e:i64
        let s_3_26: i64 = fn_state.e;
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (i128::try_from(s_3_26).unwrap());
        // D s_3_28: cast zx s_3_25 -> i
        let s_3_28: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_29: read-var result:bv
        let s_3_29: Bits = fn_state.result;
        // D s_3_30: call Elem_set(s_3_29, s_3_27, s_3_28, s_3_21)
        let s_3_30: Bits = Elem_set(state, tracer, s_3_29, s_3_27, s_3_28, s_3_21);
        // D s_3_31: write-var result <= s_3_30
        fn_state.result = s_3_30;
        // N s_3_32: branch s_3_22 b6 b4
        if s_3_22 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // C s_5_1: const #1s : i64
        let s_5_1: i64 = 1;
        // D s_5_2: add s_5_0 s_5_1
        let s_5_2: i64 = (s_5_0 + s_5_1);
        // D s_5_3: write-var e <= s_5_2
        fn_state.e = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #20696u : u32
        let s_6_0: u32 = 20696;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #20696u : u32
        let s_6_2: u32 = 20696;
        // N s_6_3: write-reg s_6_2 <= s_6_1
        let s_6_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_6_2 as isize, s_6_1);
            tracer.write_register(s_6_2 as isize, s_6_1);
        };
        // N s_6_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var datasizeshadow#1972:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1972;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var part:i64
        let s_7_5: i64 = fn_state.part;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: cast zx s_7_2 -> i
        let s_7_7: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_8: read-var result:bv
        let s_7_8: Bits = fn_state.result;
        // D s_7_9: call Vpart_set(s_7_4, s_7_6, s_7_7, s_7_8)
        let s_7_9: () = Vpart_set(state, tracer, s_7_4, s_7_6, s_7_7, s_7_8);
        // N s_7_10: return
        return;
    }
}
