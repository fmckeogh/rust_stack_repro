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
use X_read::*;
use u__id::*;
use ROR::*;
use common::*;
pub fn ShiftReg<T: Tracer>(
    state: &mut State,
    tracer: &T,
    reg: i128,
    shiftype: u32,
    amount: i128,
    N: i64,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        Nshadow_500: i64,
        gs_26479: bool,
        result: Bits,
        reg: i128,
        shiftype: u32,
        amount: i128,
        N: i64,
    }
    let fn_state = FunctionState {
        reg,
        shiftype,
        amount,
        N,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var N:i64
        let s_0_0: i64 = fn_state.N;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var Nshadow#500 <= s_0_2
        fn_state.Nshadow_500 = s_0_2;
        // D s_0_4: read-var Nshadow#500:i64
        let s_0_4: i64 = fn_state.Nshadow_500;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: read-var reg:i
        let s_0_7: i128 = fn_state.reg;
        // D s_0_8: call X_read(s_0_7, s_0_6)
        let s_0_8: Bits = X_read(state, tracer, s_0_7, s_0_6);
        // D s_0_9: write-var result <= s_0_8
        fn_state.result = s_0_8;
        // C s_0_10: const #0u : u32
        let s_0_10: u32 = 0;
        // D s_0_11: read-var shiftype:u32
        let s_0_11: u32 = fn_state.shiftype;
        // D s_0_12: cmp-eq s_0_10 s_0_11
        let s_0_12: bool = ((s_0_10) == (s_0_11));
        // D s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b3 b1
        if s_0_13 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var result:bv
        let s_1_0: Bits = fn_state.result;
        // D s_1_1: read-var amount:i
        let s_1_1: i128 = fn_state.amount;
        // D s_1_2: lsl s_1_0 s_1_1
        let s_1_2: Bits = s_1_0 << s_1_1;
        // D s_1_3: write-var result <= s_1_2
        fn_state.result = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var result:bv
        let s_2_0: Bits = fn_state.result;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #1u : u32
        let s_3_0: u32 = 1;
        // D s_3_1: read-var shiftype:u32
        let s_3_1: u32 = fn_state.shiftype;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var result:bv
        let s_4_0: Bits = fn_state.result;
        // D s_4_1: read-var amount:i
        let s_4_1: i128 = fn_state.amount;
        // D s_4_2: lsr s_4_0 s_4_1
        let s_4_2: Bits = s_4_0 >> s_4_1;
        // D s_4_3: write-var result <= s_4_2
        fn_state.result = s_4_2;
        // N s_4_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_5_0: const #2u : u32
        let s_5_0: u32 = 2;
        // D s_5_1: read-var shiftype:u32
        let s_5_1: u32 = fn_state.shiftype;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var result:bv
        let s_6_0: Bits = fn_state.result;
        // D s_6_1: read-var amount:i
        let s_6_1: i128 = fn_state.amount;
        // D s_6_2: asr s_6_0 s_6_1
        let s_6_2: Bits = s_6_0 >> s_6_1;
        // D s_6_3: write-var result <= s_6_2
        fn_state.result = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_7_0: const #3u : u32
        let s_7_0: u32 = 3;
        // D s_7_1: read-var shiftype:u32
        let s_7_1: u32 = fn_state.shiftype;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b12 b8
        if s_7_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var Nshadow#500:i64
        let s_8_0: i64 = fn_state.Nshadow_500;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #0s : i
        let s_8_4: i128 = 0;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: cmp-gt s_8_5 s_8_4
        let s_8_6: bool = ((s_8_5) > (s_8_4));
        // N s_8_7: branch s_8_6 b11 b9
        if s_8_6 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#26479 <= s_9_0
        fn_state.gs_26479 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var gs#26479:u8
        let s_10_0: bool = fn_state.gs_26479;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // D s_10_2: read-var result:bv
        let s_10_2: Bits = fn_state.result;
        // D s_10_3: read-var amount:i
        let s_10_3: i128 = fn_state.amount;
        // D s_10_4: call ROR(s_10_2, s_10_3)
        let s_10_4: Bits = ROR(state, tracer, s_10_2, s_10_3);
        // D s_10_5: write-var result <= s_10_4
        fn_state.result = s_10_4;
        // N s_10_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_11_0: read-var amount:i
        let s_11_0: i128 = fn_state.amount;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // C s_11_2: const #0s : i
        let s_11_2: i128 = 0;
        // D s_11_3: cmp-ge s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) >= (s_11_2));
        // D s_11_4: write-var gs#26479 <= s_11_3
        fn_state.gs_26479 = s_11_3;
        // N s_11_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_12_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
