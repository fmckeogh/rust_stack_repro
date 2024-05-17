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
use asl_Int::*;
use SatQ::*;
use V_read::*;
use Elem_read::*;
use Elem_set::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_add_saturating_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        esizeshadow_1991: i64,
        ga_269713: ProductTypef506aa96a892fe52,
        gs_173671: i64,
        result: Bits,
        datasizeshadow_1992: i64,
        operand2: Bits,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
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
        // D s_0_3: write-var esizeshadow#1991 <= s_0_2
        fn_state.esizeshadow_1991 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1992 <= s_0_6
        fn_state.datasizeshadow_1992 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1992:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1992;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand <= s_1_5
        fn_state.operand = s_1_5;
        // D s_1_7: read-var datasizeshadow#1992:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1992;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var d:i64
        let s_1_10: i64 = fn_state.d;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // C s_1_14: const #0s : i64
        let s_1_14: i64 = 0;
        // C s_1_15: const #1s : i
        let s_1_15: i128 = 1;
        // D s_1_16: read-var elements:i
        let s_1_16: i128 = fn_state.elements;
        // D s_1_17: sub s_1_16 s_1_15
        let s_1_17: i128 = ((s_1_16) - (s_1_15));
        // D s_1_18: cast reint s_1_17 -> i64
        let s_1_18: i64 = (s_1_17 as i64);
        // D s_1_19: write-var gs#173671 <= s_1_18
        fn_state.gs_173671 = s_1_18;
        // D s_1_20: write-var e <= s_1_14
        fn_state.e = s_1_14;
        // N s_1_21: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#173671:i64
        let s_2_1: i64 = fn_state.gs_173671;
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
        // D s_3_0: read-var esizeshadow#1991:i64
        let s_3_0: i64 = fn_state.esizeshadow_1991;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var e:i64
        let s_3_3: i64 = fn_state.e;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: read-var operand:bv
        let s_3_6: Bits = fn_state.operand;
        // D s_3_7: call Elem_read(s_3_6, s_3_4, s_3_5)
        let s_3_7: Bits = Elem_read(state, tracer, s_3_6, s_3_4, s_3_5);
        // D s_3_8: read-var is_unsigned:u8
        let s_3_8: bool = fn_state.is_unsigned;
        // D s_3_9: not s_3_8
        let s_3_9: bool = !s_3_8;
        // D s_3_10: call asl_Int(s_3_7, s_3_9)
        let s_3_10: i128 = asl_Int(state, tracer, s_3_7, s_3_9);
        // D s_3_11: read-var esizeshadow#1991:i64
        let s_3_11: i64 = fn_state.esizeshadow_1991;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // D s_3_14: read-var e:i64
        let s_3_14: i64 = fn_state.e;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: cast zx s_3_13 -> i
        let s_3_16: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_17: read-var operand2:bv
        let s_3_17: Bits = fn_state.operand2;
        // D s_3_18: call Elem_read(s_3_17, s_3_15, s_3_16)
        let s_3_18: Bits = Elem_read(state, tracer, s_3_17, s_3_15, s_3_16);
        // D s_3_19: read-var is_unsigned:u8
        let s_3_19: bool = fn_state.is_unsigned;
        // D s_3_20: call asl_Int(s_3_18, s_3_19)
        let s_3_20: i128 = asl_Int(state, tracer, s_3_18, s_3_19);
        // D s_3_21: add s_3_10 s_3_20
        let s_3_21: i128 = (s_3_10 + s_3_20);
        // D s_3_22: read-var esizeshadow#1991:i64
        let s_3_22: i64 = fn_state.esizeshadow_1991;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: cast zx s_3_24 -> i
        let s_3_25: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_26: read-var is_unsigned:u8
        let s_3_26: bool = fn_state.is_unsigned;
        // D s_3_27: call SatQ(s_3_21, s_3_25, s_3_26)
        let s_3_27: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_3_21,
            s_3_25,
            s_3_26,
        );
        // D s_3_28: write-var ga#269713 <= s_3_27
        fn_state.ga_269713 = s_3_27;
        // D s_3_29: read-var ga#269713.0:struct
        let s_3_29: Bits = fn_state.ga_269713._0;
        // D s_3_30: read-var ga#269713.1:struct
        let s_3_30: bool = fn_state.ga_269713._1;
        // D s_3_31: read-var esizeshadow#1991:i64
        let s_3_31: i64 = fn_state.esizeshadow_1991;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: cast reint s_3_32 -> i64
        let s_3_33: i64 = (s_3_32 as i64);
        // D s_3_34: read-var e:i64
        let s_3_34: i64 = fn_state.e;
        // D s_3_35: cast zx s_3_34 -> i
        let s_3_35: i128 = (i128::try_from(s_3_34).unwrap());
        // D s_3_36: cast zx s_3_33 -> i
        let s_3_36: i128 = (i128::try_from(s_3_33).unwrap());
        // D s_3_37: read-var result:bv
        let s_3_37: Bits = fn_state.result;
        // D s_3_38: call Elem_set(s_3_37, s_3_35, s_3_36, s_3_29)
        let s_3_38: Bits = Elem_set(state, tracer, s_3_37, s_3_35, s_3_36, s_3_29);
        // D s_3_39: write-var result <= s_3_38
        fn_state.result = s_3_38;
        // N s_3_40: branch s_3_30 b6 b4
        if s_3_30 {
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
        // D s_7_0: read-var datasizeshadow#1992:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1992;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var result:bv
        let s_7_5: Bits = fn_state.result;
        // D s_7_6: call V_set(s_7_4, s_7_2, s_7_5)
        let s_7_6: () = V_set(state, tracer, s_7_4, s_7_2, s_7_5);
        // N s_7_7: return
        return;
    }
}
