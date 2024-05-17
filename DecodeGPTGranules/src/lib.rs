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
use GPIValid::*;
use Unreachable::*;
use common::*;
pub fn DecodeGPTGranules<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pgs: u32,
    index: i128,
    entry: u64,
) -> ProductType5b104d2f9e197511 {
    #[derive(Default)]
    struct FunctionState {
        result: ProductType9799615a3dcac2c0,
        i: i64,
        return_value: ProductType5b104d2f9e197511,
        pgs: u32,
        index: i128,
        entry: u64,
    }
    let fn_state = FunctionState {
        pgs,
        index,
        entry,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_0_0: const #0s : i64
        let s_0_0: i64 = 0;
        // D s_0_1: write-var i <= s_0_0
        fn_state.i = s_0_0;
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #15s : i64
        let s_1_1: i64 = 15;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b6 b2
        if s_1_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var i:i64
        let s_2_1: i64 = fn_state.i;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: mul s_2_2 s_2_0
        let s_2_3: i128 = ((s_2_2) * (s_2_0));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // C s_2_5: const #4s : i
        let s_2_5: i128 = 4;
        // D s_2_6: read-var entry:u64
        let s_2_6: u64 = fn_state.entry;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 64u16);
        // D s_2_8: cast zx s_2_4 -> i
        let s_2_8: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_9: bit-extract s_2_7 s_2_8 s_2_5
        let s_2_9: Bits = (Bits::new(
            ((s_2_7) >> (s_2_8)).value(),
            u16::try_from(s_2_5).unwrap(),
        ));
        // D s_2_10: cast reint s_2_9 -> u8
        let s_2_10: u8 = (s_2_9.value() as u8);
        // D s_2_11: call GPIValid(s_2_10)
        let s_2_11: bool = GPIValid(state, tracer, s_2_10);
        // D s_2_12: not s_2_11
        let s_2_12: bool = !s_2_11;
        // N s_2_13: branch s_2_12 b4 b3
        if s_2_12 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // D s_3_0: read-var i:i64
        let s_3_0: i64 = fn_state.i;
        // C s_3_1: const #1s : i64
        let s_3_1: i64 = 1;
        // D s_3_2: add s_3_0 s_3_1
        let s_3_2: i64 = (s_3_0 + s_3_1);
        // D s_3_3: write-var i <= s_3_2
        fn_state.i = s_3_2;
        // N s_3_4: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_4_0: const #2u : u32
        let s_4_0: u32 = 2;
        // D s_4_1: read-var result:struct
        let s_4_1: ProductType9799615a3dcac2c0 = fn_state.result;
        // D s_4_2: create-product struct = ["s_4_0", "s_4_1"]
        let s_4_2: ProductType5b104d2f9e197511 = ProductType5b104d2f9e197511 {
            _0: s_4_0,
            _1: s_4_1,
        };
        // D s_4_3: write-var return_value <= s_4_2
        fn_state.return_value = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // D s_5_0: read-var return_value:struct
        let s_5_0: ProductType5b104d2f9e197511 = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_6_0: const #4s : i
        let s_6_0: i128 = 4;
        // D s_6_1: read-var index:i
        let s_6_1: i128 = fn_state.index;
        // D s_6_2: mul s_6_1 s_6_0
        let s_6_2: i128 = ((s_6_1) * (s_6_0));
        // C s_6_3: const #4s : i
        let s_6_3: i128 = 4;
        // D s_6_4: read-var entry:u64
        let s_6_4: u64 = fn_state.entry;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 64u16);
        // D s_6_6: bit-extract s_6_5 s_6_2 s_6_3
        let s_6_6: Bits = (Bits::new(
            ((s_6_5) >> (s_6_2)).value(),
            u16::try_from(s_6_3).unwrap(),
        ));
        // D s_6_7: cast reint s_6_6 -> u8
        let s_6_7: u8 = (s_6_6.value() as u8);
        // D s_6_8: write-var result.1 <= s_6_7
        fn_state.result._1 = s_6_7;
        // C s_6_9: const #0u : u32
        let s_6_9: u32 = 0;
        // D s_6_10: read-var pgs:u32
        let s_6_10: u32 = fn_state.pgs;
        // D s_6_11: cmp-eq s_6_9 s_6_10
        let s_6_11: bool = ((s_6_9) == (s_6_10));
        // D s_6_12: not s_6_11
        let s_6_12: bool = !s_6_11;
        // N s_6_13: branch s_6_12 b9 b7
        if s_6_12 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_7_0: const #912u : u32
        let s_7_0: u32 = 912;
        // D s_7_1: read-reg s_7_0:i64
        let s_7_1: i64 = {
            let value = state.read_register::<i64>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: write-var result.4 <= s_7_2
        fn_state.result._4 = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // D s_8_0: read-var result.4:struct
        let s_8_0: i128 = fn_state.result._4;
        // D s_8_1: write-var result.0 <= s_8_0
        fn_state.result._0 = s_8_0;
        // C s_8_2: const #1s : i
        let s_8_2: i128 = 1;
        // D s_8_3: write-var result.2 <= s_8_2
        fn_state.result._2 = s_8_2;
        // C s_8_4: const #0u : u32
        let s_8_4: u32 = 0;
        // D s_8_5: read-var result:struct
        let s_8_5: ProductType9799615a3dcac2c0 = fn_state.result;
        // D s_8_6: create-product struct = ["s_8_4", "s_8_5"]
        let s_8_6: ProductType5b104d2f9e197511 = ProductType5b104d2f9e197511 {
            _0: s_8_4,
            _1: s_8_5,
        };
        // D s_8_7: write-var return_value <= s_8_6
        fn_state.return_value = s_8_6;
        // N s_8_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_9_0: const #1u : u32
        let s_9_0: u32 = 1;
        // D s_9_1: read-var pgs:u32
        let s_9_1: u32 = fn_state.pgs;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_10_0: const #920u : u32
        let s_10_0: u32 = 920;
        // D s_10_1: read-reg s_10_0:i64
        let s_10_1: i64 = {
            let value = state.read_register::<i64>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: write-var result.4 <= s_10_2
        fn_state.result._4 = s_10_2;
        // N s_10_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_11_0: const #2u : u32
        let s_11_0: u32 = 2;
        // D s_11_1: read-var pgs:u32
        let s_11_1: u32 = fn_state.pgs;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_12_0: const #928u : u32
        let s_12_0: u32 = 928;
        // D s_12_1: read-reg s_12_0:i64
        let s_12_1: i64 = {
            let value = state.read_register::<i64>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: write-var result.4 <= s_12_2
        fn_state.result._4 = s_12_2;
        // N s_12_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType5b104d2f9e197511 {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call Unreachable(s_13_0)
        let s_13_1: () = Unreachable(state, tracer, s_13_0);
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
