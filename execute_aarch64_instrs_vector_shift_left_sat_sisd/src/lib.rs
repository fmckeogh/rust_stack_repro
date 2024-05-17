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
use SatQ::*;
use Elem_set::*;
use u__id::*;
use V_read::*;
use u_shl_int_general::*;
use V_set::*;
use asl_Int::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_shift_left_sat_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    dst_unsigned: bool,
    elements: i128,
    esize: i64,
    n: i64,
    shift: i128,
    src_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        e: i64,
        datasizeshadow_1956: i64,
        gs_172052: bool,
        gs_172050: bool,
        gs_172048: bool,
        result: Bits,
        esizeshadow_1955: i64,
        gs_172054: bool,
        ga_268434: ProductTypef506aa96a892fe52,
        gs_172060: i64,
        d: i64,
        datasize: i64,
        dst_unsigned: bool,
        elements: i128,
        esize: i64,
        n: i64,
        shift: i128,
        src_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        dst_unsigned,
        elements,
        esize,
        n,
        shift,
        src_unsigned,
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
        // D s_0_3: write-var esizeshadow#1955 <= s_0_2
        fn_state.esizeshadow_1955 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1956 <= s_0_6
        fn_state.datasizeshadow_1956 = s_0_6;
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
        // D s_1_0: read-var datasizeshadow#1956:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1956;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call __id(s_1_1)
        let s_1_2: i128 = u__id(state, tracer, s_1_1);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #8s : i
        let s_1_4: i128 = 8;
        // D s_1_5: cast zx s_1_3 -> i
        let s_1_5: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_6: cmp-eq s_1_5 s_1_4
        let s_1_6: bool = ((s_1_5) == (s_1_4));
        // N s_1_7: branch s_1_6 b19 b2
        if s_1_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1956:i64
        let s_2_0: i64 = fn_state.datasizeshadow_1956;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call __id(s_2_1)
        let s_2_2: i128 = u__id(state, tracer, s_2_1);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #16s : i
        let s_2_4: i128 = 16;
        // D s_2_5: cast zx s_2_3 -> i
        let s_2_5: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_6: cmp-eq s_2_5 s_2_4
        let s_2_6: bool = ((s_2_5) == (s_2_4));
        // D s_2_7: write-var gs#172048 <= s_2_6
        fn_state.gs_172048 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#172048:u8
        let s_3_0: bool = fn_state.gs_172048;
        // N s_3_1: branch s_3_0 b18 b4
        if s_3_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1956:i64
        let s_4_0: i64 = fn_state.datasizeshadow_1956;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call __id(s_4_1)
        let s_4_2: i128 = u__id(state, tracer, s_4_1);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #32s : i
        let s_4_4: i128 = 32;
        // D s_4_5: cast zx s_4_3 -> i
        let s_4_5: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_6: cmp-eq s_4_5 s_4_4
        let s_4_6: bool = ((s_4_5) == (s_4_4));
        // D s_4_7: write-var gs#172050 <= s_4_6
        fn_state.gs_172050 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#172050:u8
        let s_5_0: bool = fn_state.gs_172050;
        // N s_5_1: branch s_5_0 b17 b6
        if s_5_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var datasizeshadow#1956:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1956;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #64s : i
        let s_6_4: i128 = 64;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // D s_6_7: write-var gs#172052 <= s_6_6
        fn_state.gs_172052 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#172052:u8
        let s_7_0: bool = fn_state.gs_172052;
        // N s_7_1: branch s_7_0 b16 b8
        if s_7_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1956:i64
        let s_8_0: i64 = fn_state.datasizeshadow_1956;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #128s : i
        let s_8_4: i128 = 128;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: cmp-eq s_8_5 s_8_4
        let s_8_6: bool = ((s_8_5) == (s_8_4));
        // D s_8_7: write-var gs#172054 <= s_8_6
        fn_state.gs_172054 = s_8_6;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#172054:u8
        let s_9_0: bool = fn_state.gs_172054;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var datasizeshadow#1956:i64
        let s_9_2: i64 = fn_state.datasizeshadow_1956;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // D s_9_5: read-var n:i64
        let s_9_5: i64 = fn_state.n;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: call V_read(s_9_6, s_9_4)
        let s_9_7: Bits = V_read(state, tracer, s_9_6, s_9_4);
        // D s_9_8: write-var operand <= s_9_7
        fn_state.operand = s_9_7;
        // C s_9_9: const #0s : i64
        let s_9_9: i64 = 0;
        // C s_9_10: const #1s : i
        let s_9_10: i128 = 1;
        // D s_9_11: read-var elements:i
        let s_9_11: i128 = fn_state.elements;
        // D s_9_12: sub s_9_11 s_9_10
        let s_9_12: i128 = ((s_9_11) - (s_9_10));
        // D s_9_13: cast reint s_9_12 -> i64
        let s_9_13: i64 = (s_9_12 as i64);
        // D s_9_14: write-var gs#172060 <= s_9_13
        fn_state.gs_172060 = s_9_13;
        // D s_9_15: write-var e <= s_9_9
        fn_state.e = s_9_9;
        // N s_9_16: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: read-var gs#172060:i64
        let s_10_1: i64 = fn_state.gs_172060;
        // D s_10_2: cmp-gt s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) > (s_10_1));
        // N s_10_3: branch s_10_2 b15 b11
        if s_10_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#1955:i64
        let s_11_0: i64 = fn_state.esizeshadow_1955;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var e:i64
        let s_11_3: i64 = fn_state.e;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cast zx s_11_2 -> i
        let s_11_5: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_6: read-var operand:bv
        let s_11_6: Bits = fn_state.operand;
        // D s_11_7: call Elem_read(s_11_6, s_11_4, s_11_5)
        let s_11_7: Bits = Elem_read(state, tracer, s_11_6, s_11_4, s_11_5);
        // D s_11_8: read-var src_unsigned:u8
        let s_11_8: bool = fn_state.src_unsigned;
        // D s_11_9: call asl_Int(s_11_7, s_11_8)
        let s_11_9: i128 = asl_Int(state, tracer, s_11_7, s_11_8);
        // D s_11_10: read-var shift:i
        let s_11_10: i128 = fn_state.shift;
        // D s_11_11: call _shl_int_general(s_11_9, s_11_10)
        let s_11_11: i128 = u_shl_int_general(state, tracer, s_11_9, s_11_10);
        // D s_11_12: read-var esizeshadow#1955:i64
        let s_11_12: i64 = fn_state.esizeshadow_1955;
        // D s_11_13: cast zx s_11_12 -> i
        let s_11_13: i128 = (i128::try_from(s_11_12).unwrap());
        // D s_11_14: cast reint s_11_13 -> i64
        let s_11_14: i64 = (s_11_13 as i64);
        // D s_11_15: cast zx s_11_14 -> i
        let s_11_15: i128 = (i128::try_from(s_11_14).unwrap());
        // D s_11_16: read-var dst_unsigned:u8
        let s_11_16: bool = fn_state.dst_unsigned;
        // D s_11_17: call SatQ(s_11_11, s_11_15, s_11_16)
        let s_11_17: ProductTypef506aa96a892fe52 = SatQ(
            state,
            tracer,
            s_11_11,
            s_11_15,
            s_11_16,
        );
        // D s_11_18: write-var ga#268434 <= s_11_17
        fn_state.ga_268434 = s_11_17;
        // D s_11_19: read-var ga#268434.0:struct
        let s_11_19: Bits = fn_state.ga_268434._0;
        // D s_11_20: read-var ga#268434.1:struct
        let s_11_20: bool = fn_state.ga_268434._1;
        // D s_11_21: read-var esizeshadow#1955:i64
        let s_11_21: i64 = fn_state.esizeshadow_1955;
        // D s_11_22: cast zx s_11_21 -> i
        let s_11_22: i128 = (i128::try_from(s_11_21).unwrap());
        // D s_11_23: cast reint s_11_22 -> i64
        let s_11_23: i64 = (s_11_22 as i64);
        // D s_11_24: read-var e:i64
        let s_11_24: i64 = fn_state.e;
        // D s_11_25: cast zx s_11_24 -> i
        let s_11_25: i128 = (i128::try_from(s_11_24).unwrap());
        // D s_11_26: cast zx s_11_23 -> i
        let s_11_26: i128 = (i128::try_from(s_11_23).unwrap());
        // D s_11_27: read-var result:bv
        let s_11_27: Bits = fn_state.result;
        // D s_11_28: call Elem_set(s_11_27, s_11_25, s_11_26, s_11_19)
        let s_11_28: Bits = Elem_set(state, tracer, s_11_27, s_11_25, s_11_26, s_11_19);
        // D s_11_29: write-var result <= s_11_28
        fn_state.result = s_11_28;
        // N s_11_30: branch s_11_20 b14 b12
        if s_11_20 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var e:i64
        let s_13_0: i64 = fn_state.e;
        // C s_13_1: const #1s : i64
        let s_13_1: i64 = 1;
        // D s_13_2: add s_13_0 s_13_1
        let s_13_2: i64 = (s_13_0 + s_13_1);
        // D s_13_3: write-var e <= s_13_2
        fn_state.e = s_13_2;
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #20696u : u32
        let s_14_0: u32 = 20696;
        // D s_14_1: read-reg s_14_0:struct
        let s_14_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #20696u : u32
        let s_14_2: u32 = 20696;
        // N s_14_3: write-reg s_14_2 <= s_14_1
        let s_14_3: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_14_2 as isize, s_14_1);
            tracer.write_register(s_14_2 as isize, s_14_1);
        };
        // N s_14_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var datasizeshadow#1956:i64
        let s_15_0: i64 = fn_state.datasizeshadow_1956;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // D s_15_3: read-var d:i64
        let s_15_3: i64 = fn_state.d;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: read-var result:bv
        let s_15_5: Bits = fn_state.result;
        // D s_15_6: call V_set(s_15_4, s_15_2, s_15_5)
        let s_15_6: () = V_set(state, tracer, s_15_4, s_15_2, s_15_5);
        // N s_15_7: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#172054 <= s_16_0
        fn_state.gs_172054 = s_16_0;
        // N s_16_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#172052 <= s_17_0
        fn_state.gs_172052 = s_17_0;
        // N s_17_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#172050 <= s_18_0
        fn_state.gs_172050 = s_18_0;
        // N s_18_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var gs#172048 <= s_19_0
        fn_state.gs_172048 = s_19_0;
        // N s_19_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
