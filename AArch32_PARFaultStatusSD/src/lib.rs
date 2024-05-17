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
use IsExternalAbort__1::*;
use Bit::*;
use EncodeSDFSC::*;
use common::*;
pub fn AArch32_PARFaultStatusSD<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault: ProductType1d757adad216cdef,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        ga_23616: bool,
        syndrome: u8,
        fault: ProductType1d757adad216cdef,
    }
    let fn_state = FunctionState {
        fault,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var fault:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_0_1: call IsExternalAbort__1(s_0_0)
        let s_0_1: bool = IsExternalAbort__1(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var ga#23616 <= s_1_0
        fn_state.ga_23616 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var ga#23616:u8
        let s_2_0: bool = fn_state.ga_23616;
        // D s_2_1: call Bit(s_2_0)
        let s_2_1: bool = Bit(state, tracer, s_2_0);
        // C s_2_2: const #5s : i
        let s_2_2: i128 = 5;
        // D s_2_3: read-var syndrome:u8
        let s_2_3: u8 = fn_state.syndrome;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 6u16);
        // C s_2_5: const #1u : u64
        let s_2_5: u64 = 1;
        // D s_2_6: bit-insert s_2_4 s_2_4 s_2_2 s_2_5
        let s_2_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_4.length(),
            );
            (s_2_4 & mask) | (s_2_4 << s_2_2)
        };
        // D s_2_7: cast reint s_2_6 -> u8
        let s_2_7: u8 = (s_2_6.value() as u8);
        // D s_2_8: write-var syndrome <= s_2_7
        fn_state.syndrome = s_2_7;
        // D s_2_9: read-var fault.16:struct
        let s_2_9: u32 = fn_state.fault._16;
        // D s_2_10: read-var fault.9:struct
        let s_2_10: i128 = fn_state.fault._9;
        // D s_2_11: call EncodeSDFSC(s_2_9, s_2_10)
        let s_2_11: u8 = EncodeSDFSC(state, tracer, s_2_9, s_2_10);
        // C s_2_12: const #0s : i
        let s_2_12: i128 = 0;
        // D s_2_13: cast zx s_2_7 -> bv
        let s_2_13: Bits = Bits::new(s_2_7 as u128, 6u16);
        // D s_2_14: cast zx s_2_11 -> bv
        let s_2_14: Bits = Bits::new(s_2_11 as u128, 5u16);
        // C s_2_15: const #4s : i
        let s_2_15: i128 = 4;
        // C s_2_16: const #1u : u64
        let s_2_16: u64 = 1;
        // C s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 64u16);
        // C s_2_18: lsl s_2_17 s_2_15
        let s_2_18: Bits = s_2_17 << s_2_15;
        // C s_2_19: sub s_2_18 s_2_17
        let s_2_19: Bits = ((s_2_18) - (s_2_17));
        // D s_2_20: and s_2_14 s_2_19
        let s_2_20: Bits = ((s_2_14) & (s_2_19));
        // D s_2_21: lsl s_2_20 s_2_12
        let s_2_21: Bits = s_2_20 << s_2_12;
        // C s_2_22: lsl s_2_19 s_2_12
        let s_2_22: Bits = s_2_19 << s_2_12;
        // C s_2_23: cmpl s_2_22
        let s_2_23: Bits = !s_2_22;
        // D s_2_24: and s_2_13 s_2_23
        let s_2_24: Bits = ((s_2_13) & (s_2_23));
        // D s_2_25: or s_2_24 s_2_21
        let s_2_25: Bits = ((s_2_24) | (s_2_21));
        // D s_2_26: cast reint s_2_25 -> u8
        let s_2_26: u8 = (s_2_25.value() as u8);
        // D s_2_27: write-var syndrome <= s_2_26
        fn_state.syndrome = s_2_26;
        // N s_2_28: return s_2_26
        return s_2_26;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var fault.5:struct
        let s_3_0: bool = fn_state.fault._5;
        // D s_3_1: write-var ga#23616 <= s_3_0
        fn_state.ga_23616 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
