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
use AArch32_FullTranslate::*;
use RegimeUsingAArch32::*;
use IsFault::*;
use TranslationRegime::*;
use AArch64_TranslateAddress::*;
use AArch32_CheckDebug::*;
use common::*;
pub fn AArch32_TranslateAddress<T: Tracer>(
    state: &mut State,
    tracer: &T,
    va: u32,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
    size: i128,
) -> ProductTypece7c66ccb2cab13e {
    #[derive(Default)]
    struct FunctionState {
        ga_23451: ProductTypece7c66ccb2cab13e,
        result: ProductTypece7c66ccb2cab13e,
        return_value: ProductTypece7c66ccb2cab13e,
        va: u32,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
        size: i128,
    }
    let fn_state = FunctionState {
        va,
        accdesc,
        aligned,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call TranslationRegime(s_0_1)
        let s_0_2: u32 = TranslationRegime(state, tracer, s_0_1);
        // D s_0_3: call RegimeUsingAArch32(s_0_2)
        let s_0_3: bool = RegimeUsingAArch32(state, tracer, s_0_2);
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b7 b1
        if s_0_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_1_0: read-var va:u32
        let s_1_0: u32 = fn_state.va;
        // D s_1_1: read-var accdesc:struct
        let s_1_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_1_2: read-var aligned:u8
        let s_1_2: bool = fn_state.aligned;
        // D s_1_3: call AArch32_FullTranslate(s_1_0, s_1_1, s_1_2)
        let s_1_3: ProductTypece7c66ccb2cab13e = AArch32_FullTranslate(
            state,
            tracer,
            s_1_0,
            s_1_1,
            s_1_2,
        );
        // D s_1_4: write-var result <= s_1_3
        fn_state.result = s_1_3;
        // D s_1_5: read-var result:struct
        let s_1_5: ProductTypece7c66ccb2cab13e = fn_state.result;
        // D s_1_6: call IsFault(s_1_5)
        let s_1_6: bool = IsFault(state, tracer, s_1_5);
        // D s_1_7: not s_1_6
        let s_1_7: bool = !s_1_6;
        // N s_1_8: branch s_1_7 b5 b2
        if s_1_7 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_3_0: const #64s : i
        let s_3_0: i128 = 64;
        // D s_3_1: read-var va:u32
        let s_3_1: u32 = fn_state.va;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 32u16);
        // D s_3_3: bits-cast zx s_3_2 -> bv length s_3_0
        let s_3_3: Bits = s_3_2.zero_extend(s_3_0);
        // D s_3_4: cast reint s_3_3 -> u64
        let s_3_4: u64 = (s_3_3.value() as u64);
        // D s_3_5: write-var result.7 <= s_3_4
        fn_state.result._7 = s_3_4;
        // D s_3_6: read-var result:struct
        let s_3_6: ProductTypece7c66ccb2cab13e = fn_state.result;
        // D s_3_7: write-var return_value <= s_3_6
        fn_state.return_value = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_4_0: read-var return_value:struct
        let s_4_0: ProductTypece7c66ccb2cab13e = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_5_0: read-var va:u32
        let s_5_0: u32 = fn_state.va;
        // D s_5_1: read-var accdesc:struct
        let s_5_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_5_2: read-var size:i
        let s_5_2: i128 = fn_state.size;
        // D s_5_3: call AArch32_CheckDebug(s_5_0, s_5_1, s_5_2)
        let s_5_3: ProductType1d757adad216cdef = AArch32_CheckDebug(
            state,
            tracer,
            s_5_0,
            s_5_1,
            s_5_2,
        );
        // D s_5_4: write-var result.0 <= s_5_3
        fn_state.result._0 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // N s_6_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // C s_7_0: const #64s : i
        let s_7_0: i128 = 64;
        // D s_7_1: read-var va:u32
        let s_7_1: u32 = fn_state.va;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 32u16);
        // D s_7_3: bits-cast zx s_7_2 -> bv length s_7_0
        let s_7_3: Bits = s_7_2.zero_extend(s_7_0);
        // D s_7_4: cast reint s_7_3 -> u64
        let s_7_4: u64 = (s_7_3.value() as u64);
        // D s_7_5: read-var accdesc:struct
        let s_7_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_7_6: read-var aligned:u8
        let s_7_6: bool = fn_state.aligned;
        // D s_7_7: read-var size:i
        let s_7_7: i128 = fn_state.size;
        // D s_7_8: call AArch64_TranslateAddress(s_7_4, s_7_5, s_7_6, s_7_7)
        let s_7_8: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_7_4,
            s_7_5,
            s_7_6,
            s_7_7,
        );
        // D s_7_9: write-var ga#23451 <= s_7_8
        fn_state.ga_23451 = s_7_8;
        // N s_7_10: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypece7c66ccb2cab13e {
        // D s_8_0: read-var ga#23451:struct
        let s_8_0: ProductTypece7c66ccb2cab13e = fn_state.ga_23451;
        // D s_8_1: write-var return_value <= s_8_0
        fn_state.return_value = s_8_0;
        // N s_8_2: jump b4
        return block_4(state, tracer, fn_state);
    }
}
