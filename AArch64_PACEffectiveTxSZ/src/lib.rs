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
use AArch64_MaxTxSZ::*;
use ConstrainUnpredictable::*;
use AArch64_S1TxSZFaults::*;
use AArch64_S1MinTxSZ::*;
use integer_subrange::*;
use common::*;
pub fn AArch64_PACEffectiveTxSZ<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        s1maxtxsz: i128,
        return_value: u8,
        s1mintxsz: i128,
        regime: u32,
        walkparams: ProductTypeef284266e139aee2,
    }
    let fn_state = FunctionState {
        regime,
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var walkparams.36:struct
        let s_0_0: u32 = fn_state.walkparams._36;
        // D s_0_1: call AArch64_MaxTxSZ(s_0_0)
        let s_0_1: i128 = AArch64_MaxTxSZ(state, tracer, s_0_0);
        // D s_0_2: write-var s1maxtxsz <= s_0_1
        fn_state.s1maxtxsz = s_0_1;
        // D s_0_3: read-var walkparams.3:struct
        let s_0_3: bool = fn_state.walkparams._3;
        // D s_0_4: read-var walkparams.7:struct
        let s_0_4: bool = fn_state.walkparams._7;
        // D s_0_5: read-var walkparams.36:struct
        let s_0_5: u32 = fn_state.walkparams._36;
        // D s_0_6: read-var regime:u32
        let s_0_6: u32 = fn_state.regime;
        // D s_0_7: call AArch64_S1MinTxSZ(s_0_6, s_0_3, s_0_4, s_0_5)
        let s_0_7: i128 = AArch64_S1MinTxSZ(state, tracer, s_0_6, s_0_3, s_0_4, s_0_5);
        // D s_0_8: write-var s1mintxsz <= s_0_7
        fn_state.s1mintxsz = s_0_7;
        // D s_0_9: read-var regime:u32
        let s_0_9: u32 = fn_state.regime;
        // D s_0_10: read-var walkparams:struct
        let s_0_10: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_0_11: call AArch64_S1TxSZFaults(s_0_9, s_0_10)
        let s_0_11: bool = AArch64_S1TxSZFaults(state, tracer, s_0_9, s_0_10);
        // N s_0_12: branch s_0_11 b8 b1
        if s_0_11 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_1_0: read-var walkparams.37:struct
        let s_1_0: u8 = fn_state.walkparams._37;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 6u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: read-var s1mintxsz:i
        let s_1_5: i128 = fn_state.s1mintxsz;
        // D s_1_6: cmp-lt s_1_4 s_1_5
        let s_1_6: bool = ((s_1_4) < (s_1_5));
        // N s_1_7: branch s_1_6 b7 b2
        if s_1_6 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var walkparams.37:struct
        let s_2_0: u8 = fn_state.walkparams._37;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 6u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: read-var s1maxtxsz:i
        let s_2_5: i128 = fn_state.s1maxtxsz;
        // D s_2_6: cmp-gt s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) > (s_2_5));
        // N s_2_7: branch s_2_6 b6 b3
        if s_2_6 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var walkparams.37:struct
        let s_4_0: u8 = fn_state.walkparams._37;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_5_0: read-var return_value:u8
        let s_5_0: u8 = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #5s : i
        let s_6_0: i128 = 5;
        // C s_6_1: const #0s : i
        let s_6_1: i128 = 0;
        // D s_6_2: read-var s1maxtxsz:i
        let s_6_2: i128 = fn_state.s1maxtxsz;
        // D s_6_3: call integer_subrange(s_6_2, s_6_0, s_6_1)
        let s_6_3: Bits = integer_subrange(state, tracer, s_6_2, s_6_0, s_6_1);
        // D s_6_4: cast reint s_6_3 -> u8
        let s_6_4: u8 = (s_6_3.value() as u8);
        // D s_6_5: write-var return_value <= s_6_4
        fn_state.return_value = s_6_4;
        // N s_6_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_7_0: const #5s : i
        let s_7_0: i128 = 5;
        // C s_7_1: const #0s : i
        let s_7_1: i128 = 0;
        // D s_7_2: read-var s1mintxsz:i
        let s_7_2: i128 = fn_state.s1mintxsz;
        // D s_7_3: call integer_subrange(s_7_2, s_7_0, s_7_1)
        let s_7_3: Bits = integer_subrange(state, tracer, s_7_2, s_7_0, s_7_1);
        // D s_7_4: cast reint s_7_3 -> u8
        let s_7_4: u8 = (s_7_3.value() as u8);
        // D s_7_5: write-var return_value <= s_7_4
        fn_state.return_value = s_7_4;
        // N s_7_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #16u : u32
        let s_8_0: u32 = 16;
        // S s_8_1: call ConstrainUnpredictable(s_8_0)
        let s_8_1: u32 = ConstrainUnpredictable(state, tracer, s_8_0);
        // C s_8_2: const #25u : u32
        let s_8_2: u32 = 25;
        // S s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // N s_8_4: branch s_8_3 b11 b9
        if s_8_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_10_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_11_0: read-var walkparams.37:struct
        let s_11_0: u8 = fn_state.walkparams._37;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 6u16);
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (s_11_1.value() as i128);
        // D s_11_3: cast reint s_11_2 -> i64
        let s_11_3: i64 = (s_11_2 as i64);
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: read-var s1mintxsz:i
        let s_11_5: i128 = fn_state.s1mintxsz;
        // D s_11_6: cmp-lt s_11_4 s_11_5
        let s_11_6: bool = ((s_11_4) < (s_11_5));
        // N s_11_7: branch s_11_6 b15 b12
        if s_11_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_12_0: read-var walkparams.37:struct
        let s_12_0: u8 = fn_state.walkparams._37;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 6u16);
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (s_12_1.value() as i128);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: read-var s1maxtxsz:i
        let s_12_5: i128 = fn_state.s1maxtxsz;
        // D s_12_6: cmp-gt s_12_4 s_12_5
        let s_12_6: bool = ((s_12_4) > (s_12_5));
        // N s_12_7: branch s_12_6 b14 b13
        if s_12_6 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_13_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_14_0: const #5s : i
        let s_14_0: i128 = 5;
        // C s_14_1: const #0s : i
        let s_14_1: i128 = 0;
        // D s_14_2: read-var s1maxtxsz:i
        let s_14_2: i128 = fn_state.s1maxtxsz;
        // D s_14_3: call integer_subrange(s_14_2, s_14_0, s_14_1)
        let s_14_3: Bits = integer_subrange(state, tracer, s_14_2, s_14_0, s_14_1);
        // D s_14_4: cast reint s_14_3 -> u8
        let s_14_4: u8 = (s_14_3.value() as u8);
        // D s_14_5: write-var return_value <= s_14_4
        fn_state.return_value = s_14_4;
        // N s_14_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #5s : i
        let s_15_0: i128 = 5;
        // C s_15_1: const #0s : i
        let s_15_1: i128 = 0;
        // D s_15_2: read-var s1mintxsz:i
        let s_15_2: i128 = fn_state.s1mintxsz;
        // D s_15_3: call integer_subrange(s_15_2, s_15_0, s_15_1)
        let s_15_3: Bits = integer_subrange(state, tracer, s_15_2, s_15_0, s_15_1);
        // D s_15_4: cast reint s_15_3 -> u8
        let s_15_4: u8 = (s_15_3.value() as u8);
        // D s_15_5: write-var return_value <= s_15_4
        fn_state.return_value = s_15_4;
        // N s_15_6: jump b5
        return block_5(state, tracer, fn_state);
    }
}
