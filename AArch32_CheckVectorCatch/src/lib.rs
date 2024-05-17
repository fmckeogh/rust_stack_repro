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
use S1TranslationRegime__1::*;
use ELUsingAArch32::*;
use AArch32_VCRMatch::*;
use ConstrainUnpredictableBool::*;
use common::*;
pub fn AArch32_CheckVectorCatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    vaddress: u32,
    size: i128,
) -> ProductType1d757adad216cdef {
    #[derive(Default)]
    struct FunctionState {
        gs_30095: bool,
        fault: ProductType1d757adad216cdef,
        gs_30097: bool,
        val_match: bool,
        fault_in: ProductType1d757adad216cdef,
        vaddress: u32,
        size: i128,
    }
    let fn_state = FunctionState {
        fault_in,
        vaddress,
        size,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call S1TranslationRegime__1(s_0_0)
        let s_0_1: u8 = S1TranslationRegime__1(state, tracer, s_0_0);
        // S s_0_2: call ELUsingAArch32(s_0_1)
        let s_0_2: bool = ELUsingAArch32(state, tracer, s_0_1);
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // D s_0_4: read-var fault_in:struct
        let s_0_4: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_5: write-var fault <= s_0_4
        fn_state.fault = s_0_4;
        // D s_0_6: read-var vaddress:u32
        let s_0_6: u32 = fn_state.vaddress;
        // D s_0_7: call AArch32_VCRMatch(s_0_6)
        let s_0_7: bool = AArch32_VCRMatch(state, tracer, s_0_6);
        // D s_0_8: write-var val_match <= s_0_7
        fn_state.val_match = s_0_7;
        // C s_0_9: const #4s : i
        let s_0_9: i128 = 4;
        // D s_0_10: read-var size:i
        let s_0_10: i128 = fn_state.size;
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // N s_0_12: branch s_0_11 b12 b1
        if s_0_11 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#30095 <= s_1_0
        fn_state.gs_30095 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_2_0: read-var gs#30095:u8
        let s_2_0: bool = fn_state.gs_30095;
        // N s_2_1: branch s_2_0 b11 b3
        if s_2_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#30097 <= s_3_0
        fn_state.gs_30097 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_4_0: read-var gs#30097:u8
        let s_4_0: bool = fn_state.gs_30097;
        // N s_4_1: branch s_4_0 b10 b5
        if s_4_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_6_0: read-var val_match:u8
        let s_6_0: bool = fn_state.val_match;
        // N s_6_1: branch s_6_0 b9 b7
        if s_6_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_8_0: read-var fault:struct
        let s_8_0: ProductType1d757adad216cdef = fn_state.fault;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_9_0: const #17u : u32
        let s_9_0: u32 = 17;
        // D s_9_1: write-var fault.16 <= s_9_0
        fn_state.fault._16 = s_9_0;
        // C s_9_2: const #1328u : u32
        let s_9_2: u32 = 1328;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: write-var fault.2 <= s_9_3
        fn_state.fault._2 = s_9_3;
        // N s_9_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_10_0: const #25u : u32
        let s_10_0: u32 = 25;
        // S s_10_1: call ConstrainUnpredictableBool(s_10_0)
        let s_10_1: bool = ConstrainUnpredictableBool(state, tracer, s_10_0);
        // D s_10_2: write-var val_match <= s_10_1
        fn_state.val_match = s_10_1;
        // N s_10_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // C s_11_0: const #2s : i
        let s_11_0: i128 = 2;
        // D s_11_1: read-var vaddress:u32
        let s_11_1: u32 = fn_state.vaddress;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 32u16);
        // C s_11_3: cast cvt s_11_0 -> bv
        let s_11_3: Bits = Bits::new(s_11_0 as u128, 128);
        // D s_11_4: add s_11_2 s_11_3
        let s_11_4: Bits = (s_11_2 + s_11_3);
        // D s_11_5: cast reint s_11_4 -> u32
        let s_11_5: u32 = (s_11_4.value() as u32);
        // D s_11_6: call AArch32_VCRMatch(s_11_5)
        let s_11_6: bool = AArch32_VCRMatch(state, tracer, s_11_5);
        // D s_11_7: write-var gs#30097 <= s_11_6
        fn_state.gs_30097 = s_11_6;
        // N s_11_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType1d757adad216cdef {
        // D s_12_0: read-var val_match:u8
        let s_12_0: bool = fn_state.val_match;
        // D s_12_1: not s_12_0
        let s_12_1: bool = !s_12_0;
        // D s_12_2: write-var gs#30095 <= s_12_1
        fn_state.gs_30095 = s_12_1;
        // N s_12_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
