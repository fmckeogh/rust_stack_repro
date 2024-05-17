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
use StandardFPSCRValue::*;
use Q_read::*;
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use Q_set::*;
use Elem_read::*;
use Zeros::*;
use FPMulAdd::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VFMA_bfs_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    elements: i64,
    i: i64,
    m: i64,
    n: i64,
    sel: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        element2: u32,
        e: i64,
        operand3: u128,
        gs_327144: i64,
        result: u128,
        operand1: u128,
        gs_917631: Bits,
        d: i64,
        elements: i64,
        i: i64,
        m: i64,
        n: i64,
        sel: i64,
    }
    let fn_state = FunctionState {
        d,
        elements,
        i,
        m,
        n,
        sel,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1s : i64
        let s_1_0: i64 = 1;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: lsr s_1_1 s_1_0
        let s_1_2: i64 = s_1_1 >> s_1_0;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call Q_read(s_1_3)
        let s_1_4: u128 = Q_read(state, tracer, s_1_3);
        // D s_1_5: write-var operand1 <= s_1_4
        fn_state.operand1 = s_1_4;
        // D s_1_6: read-var m:i64
        let s_1_6: i64 = fn_state.m;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: call D_read(s_1_7)
        let s_1_8: u64 = D_read(state, tracer, s_1_7);
        // C s_1_9: const #1s : i64
        let s_1_9: i64 = 1;
        // D s_1_10: read-var d:i64
        let s_1_10: i64 = fn_state.d;
        // D s_1_11: lsr s_1_10 s_1_9
        let s_1_11: i64 = s_1_10 >> s_1_9;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call Q_read(s_1_12)
        let s_1_13: u128 = Q_read(state, tracer, s_1_12);
        // D s_1_14: write-var operand3 <= s_1_13
        fn_state.operand3 = s_1_13;
        // C s_1_15: const #16s : i64
        let s_1_15: i64 = 16;
        // D s_1_16: cast zx s_1_8 -> bv
        let s_1_16: Bits = Bits::new(s_1_8 as u128, 64u16);
        // D s_1_17: read-var i:i64
        let s_1_17: i64 = fn_state.i;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // C s_1_19: cast zx s_1_15 -> i
        let s_1_19: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_20: call Elem_read(s_1_16, s_1_18, s_1_19)
        let s_1_20: Bits = Elem_read(state, tracer, s_1_16, s_1_18, s_1_19);
        // D s_1_21: cast reint s_1_20 -> u16
        let s_1_21: u16 = (s_1_20.value() as u16);
        // C s_1_22: const #16s : i
        let s_1_22: i128 = 16;
        // S s_1_23: call Zeros(s_1_22)
        let s_1_23: Bits = Zeros(state, tracer, s_1_22);
        // S s_1_24: cast reint s_1_23 -> u16
        let s_1_24: u16 = (s_1_23.value() as u16);
        // D s_1_25: cast zx s_1_21 -> bv
        let s_1_25: Bits = Bits::new(s_1_21 as u128, 16u16);
        // S s_1_26: cast zx s_1_24 -> bv
        let s_1_26: Bits = Bits::new(s_1_24 as u128, 16u16);
        // D s_1_27: cast reint s_1_25 -> u128
        let s_1_27: u128 = (s_1_25.value() as u128);
        // D s_1_28: size-of s_1_25
        let s_1_28: u16 = s_1_25.length();
        // S s_1_29: cast reint s_1_26 -> u128
        let s_1_29: u128 = (s_1_26.value() as u128);
        // D s_1_30: size-of s_1_26
        let s_1_30: u16 = s_1_26.length();
        // D s_1_31: lsl s_1_27 s_1_30
        let s_1_31: u128 = s_1_27 << s_1_30;
        // D s_1_32: or s_1_31 s_1_29
        let s_1_32: u128 = ((s_1_31) | (s_1_29));
        // D s_1_33: add s_1_28 s_1_30
        let s_1_33: u16 = (s_1_28 + s_1_30);
        // D s_1_34: create-bits s_1_32 s_1_33
        let s_1_34: Bits = Bits::new(s_1_32, s_1_33);
        // D s_1_35: cast reint s_1_34 -> u32
        let s_1_35: u32 = (s_1_34.value() as u32);
        // D s_1_36: write-var element2 <= s_1_35
        fn_state.element2 = s_1_35;
        // C s_1_37: const #0s : i64
        let s_1_37: i64 = 0;
        // C s_1_38: const #1s : i
        let s_1_38: i128 = 1;
        // D s_1_39: read-var elements:i64
        let s_1_39: i64 = fn_state.elements;
        // D s_1_40: cast zx s_1_39 -> i
        let s_1_40: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_41: sub s_1_40 s_1_38
        let s_1_41: i128 = ((s_1_40) - (s_1_38));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var gs#327144 <= s_1_42
        fn_state.gs_327144 = s_1_42;
        // D s_1_44: write-var e <= s_1_37
        fn_state.e = s_1_37;
        // N s_1_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#327144:i64
        let s_2_1: i64 = fn_state.gs_327144;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b5 b3
        if s_2_2 {
            return block_5(state, tracer, fn_state);
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
        // D s_3_1: read-var e:i64
        let s_3_1: i64 = fn_state.e;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: read-var sel:i64
        let s_3_6: i64 = fn_state.sel;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: add s_3_5 s_3_7
        let s_3_8: i128 = (s_3_5 + s_3_7);
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // C s_3_10: const #16s : i64
        let s_3_10: i64 = 16;
        // D s_3_11: read-var operand1:u128
        let s_3_11: u128 = fn_state.operand1;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 128u16);
        // D s_3_13: cast zx s_3_9 -> i
        let s_3_13: i128 = (i128::try_from(s_3_9).unwrap());
        // C s_3_14: cast zx s_3_10 -> i
        let s_3_14: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_15: call Elem_read(s_3_12, s_3_13, s_3_14)
        let s_3_15: Bits = Elem_read(state, tracer, s_3_12, s_3_13, s_3_14);
        // D s_3_16: cast reint s_3_15 -> u16
        let s_3_16: u16 = (s_3_15.value() as u16);
        // C s_3_17: const #16s : i
        let s_3_17: i128 = 16;
        // S s_3_18: call Zeros(s_3_17)
        let s_3_18: Bits = Zeros(state, tracer, s_3_17);
        // S s_3_19: cast reint s_3_18 -> u16
        let s_3_19: u16 = (s_3_18.value() as u16);
        // D s_3_20: cast zx s_3_16 -> bv
        let s_3_20: Bits = Bits::new(s_3_16 as u128, 16u16);
        // S s_3_21: cast zx s_3_19 -> bv
        let s_3_21: Bits = Bits::new(s_3_19 as u128, 16u16);
        // D s_3_22: cast reint s_3_20 -> u128
        let s_3_22: u128 = (s_3_20.value() as u128);
        // D s_3_23: size-of s_3_20
        let s_3_23: u16 = s_3_20.length();
        // S s_3_24: cast reint s_3_21 -> u128
        let s_3_24: u128 = (s_3_21.value() as u128);
        // D s_3_25: size-of s_3_21
        let s_3_25: u16 = s_3_21.length();
        // D s_3_26: lsl s_3_22 s_3_25
        let s_3_26: u128 = s_3_22 << s_3_25;
        // D s_3_27: or s_3_26 s_3_24
        let s_3_27: u128 = ((s_3_26) | (s_3_24));
        // D s_3_28: add s_3_23 s_3_25
        let s_3_28: u16 = (s_3_23 + s_3_25);
        // D s_3_29: create-bits s_3_27 s_3_28
        let s_3_29: Bits = Bits::new(s_3_27, s_3_28);
        // D s_3_30: cast reint s_3_29 -> u32
        let s_3_30: u32 = (s_3_29.value() as u32);
        // C s_3_31: const #32s : i64
        let s_3_31: i64 = 32;
        // D s_3_32: read-var operand3:u128
        let s_3_32: u128 = fn_state.operand3;
        // D s_3_33: cast zx s_3_32 -> bv
        let s_3_33: Bits = Bits::new(s_3_32 as u128, 128u16);
        // D s_3_34: read-var e:i64
        let s_3_34: i64 = fn_state.e;
        // D s_3_35: cast zx s_3_34 -> i
        let s_3_35: i128 = (i128::try_from(s_3_34).unwrap());
        // C s_3_36: cast zx s_3_31 -> i
        let s_3_36: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_37: call Elem_read(s_3_33, s_3_35, s_3_36)
        let s_3_37: Bits = Elem_read(state, tracer, s_3_33, s_3_35, s_3_36);
        // D s_3_38: cast reint s_3_37 -> u32
        let s_3_38: u32 = (s_3_37.value() as u32);
        // C s_3_39: const #() : ()
        let s_3_39: () = ();
        // S s_3_40: call StandardFPSCRValue(s_3_39)
        let s_3_40: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_3_39,
        );
        // D s_3_41: cast zx s_3_38 -> bv
        let s_3_41: Bits = Bits::new(s_3_38 as u128, 32u16);
        // D s_3_42: cast zx s_3_30 -> bv
        let s_3_42: Bits = Bits::new(s_3_30 as u128, 32u16);
        // D s_3_43: read-var element2:u32
        let s_3_43: u32 = fn_state.element2;
        // D s_3_44: cast zx s_3_43 -> bv
        let s_3_44: Bits = Bits::new(s_3_43 as u128, 32u16);
        // D s_3_45: call FPMulAdd(s_3_41, s_3_42, s_3_44, s_3_40)
        let s_3_45: Bits = FPMulAdd(state, tracer, s_3_41, s_3_42, s_3_44, s_3_40);
        // D s_3_46: write-var gs#917631 <= s_3_45
        fn_state.gs_917631 = s_3_45;
        // N s_3_47: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#917631:bv
        let s_4_0: Bits = fn_state.gs_917631;
        // D s_4_1: cast reint s_4_0 -> u32
        let s_4_1: u32 = (s_4_0.value() as u32);
        // D s_4_2: read-var result:u128
        let s_4_2: u128 = fn_state.result;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 128u16);
        // D s_4_4: read-var e:i64
        let s_4_4: i64 = fn_state.e;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // C s_4_6: const #32s : i64
        let s_4_6: i64 = 32;
        // C s_4_7: cast zx s_4_6 -> i
        let s_4_7: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_8: cast zx s_4_1 -> bv
        let s_4_8: Bits = Bits::new(s_4_1 as u128, 32u16);
        // D s_4_9: call Elem_set(s_4_3, s_4_5, s_4_7, s_4_8)
        let s_4_9: Bits = Elem_set(state, tracer, s_4_3, s_4_5, s_4_7, s_4_8);
        // D s_4_10: cast reint s_4_9 -> u128
        let s_4_10: u128 = (s_4_9.value() as u128);
        // D s_4_11: write-var result <= s_4_10
        fn_state.result = s_4_10;
        // D s_4_12: read-var e:i64
        let s_4_12: i64 = fn_state.e;
        // C s_4_13: const #1s : i64
        let s_4_13: i64 = 1;
        // D s_4_14: add s_4_12 s_4_13
        let s_4_14: i64 = (s_4_12 + s_4_13);
        // D s_4_15: write-var e <= s_4_14
        fn_state.e = s_4_14;
        // N s_4_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #1s : i64
        let s_5_0: i64 = 1;
        // D s_5_1: read-var d:i64
        let s_5_1: i64 = fn_state.d;
        // D s_5_2: lsr s_5_1 s_5_0
        let s_5_2: i64 = s_5_1 >> s_5_0;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var result:u128
        let s_5_4: u128 = fn_state.result;
        // D s_5_5: call Q_set(s_5_3, s_5_4)
        let s_5_5: () = Q_set(state, tracer, s_5_3, s_5_4);
        // N s_5_6: return
        return;
    }
}
